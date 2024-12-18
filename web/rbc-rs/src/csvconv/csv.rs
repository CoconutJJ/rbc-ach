use crate::lib::error::ErrorLog;
use crate::lib::header::CPA005Record;
use crate::lib::payment::{BasicPayment, BasicPaymentSegment};
use crate::lib::types::{CurrencyType, ProcessingCentre, RecordType};
use chrono::{Datelike, NaiveDate};
use csv::{Reader, ReaderBuilder, StringRecord};
use serde::Deserialize;

fn validate_csv_header<'a>(
    rdr: &'a mut Reader<&[u8]>,
    header_name: &str,
) -> Result<String, String> {
    let mut record = StringRecord::new();

    match rdr.read_record(&mut record) {
        Ok(true) => (),
        _ => {
            return Err(format!(
                "Could not read CSV header record: {}\n",
                header_name
            ))
        }
    }

    let header = record.get(0);

    match header {
        Some(s) => {
            if s.trim() != header_name {
                return Err(format!(
                    "Expected header {}, got {} instead\n",
                    header_name, s
                ));
            }
        }

        None => {
            return Err("No header found!\n".to_string());
        }
    }

    let value = record.get(1);

    match value {
        Some(s) => return Ok(s.to_string()),
        None => return Err(format!("Expected value for header {}\n", header_name)),
    }
}

#[derive(Debug)]
struct CSVHeader {
    client_name: String,
    client_number: String,
    processing_centre: ProcessingCentre,
    currency_code: CurrencyType,
    payment_date: (u64, u64),
    transaction_code: String,
}

impl CSVHeader {
    fn new() -> Self {
        Self {
            client_name: String::new(),
            client_number: String::new(),
            processing_centre: ProcessingCentre::Vancouver,
            currency_code: CurrencyType::CAD,
            payment_date: (0, 0),
            transaction_code: String::new(),
        }
    }
}

fn parse_dollar_amount_to_cents(amount: &String) -> Option<u64> {

    let mut sanitized_amount = String::new();

    for c in amount.chars() {
        if c == '.' {
            sanitized_amount.push(c);
        } else if ('0' as u8) <= (c as u8) && (c as u8) <= ('9' as u8) {
            sanitized_amount.push(c);
        } else if c == ',' || c == ' ' || c == '$' {
            continue;
        } else {
            println!("Error {}", c);
            return None;
        }
    }

    match sanitized_amount.parse::<f64>() {
        Ok(s) => return Some((s * 100.0).round() as u64),
        Err(_) => return None,
    }
}

#[derive(Deserialize, Debug)]
struct CSVRow {
    customer_number: String,
    customer_name: String,
    bank: String,
    branch: String,
    account: String,
    amount: String,
    suspend: String,
    _todo: String,
    _total: String,
}

pub fn convert_to_cpa005(csv: String, record_type: RecordType) -> Result<String, ErrorLog> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv.as_bytes());

    let mut csv_header = CSVHeader::new();
    let mut errors = ErrorLog::new();

    match validate_csv_header(&mut rdr, "Client Name") {
        Ok(s) => {
            csv_header.client_name = s.to_string();
        }
        Err(s) => {
            errors.write_error(s.as_str());
        }
    }

    match validate_csv_header(&mut rdr, "Client Number") {
        Ok(s) => {
            csv_header.client_number = s;
        }
        Err(s) => {
            errors.write_error(s.as_str());
        }
    }

    match validate_csv_header(&mut rdr, "Processing Centre") {
        Ok(s) => {
            csv_header.processing_centre = match format!("{:0>5}", s).as_str() {
                "00330" => ProcessingCentre::Halifax,
                "00310" => ProcessingCentre::Montreal,
                "00320" => ProcessingCentre::Toronto,
                "00278" => ProcessingCentre::Regina,
                "00370" => ProcessingCentre::Winnipeg,
                "00390" => ProcessingCentre::Calgary,
                "00300" => ProcessingCentre::Vancouver,
                s => {
                    errors.write_error(
                        format!("Invalid Processing Centre: {} specified in CSV header\n", s)
                            .as_str(),
                    );
                    ProcessingCentre::Vancouver
                }
            }
        }
        Err(s) => {
            errors.write_error(s.as_str());
        }
    }

    match validate_csv_header(&mut rdr, "Currency Code") {
        Ok(s) => {
            csv_header.currency_code = match s.to_uppercase().as_str() {
                "CAD" => CurrencyType::CAD,
                "USD" => CurrencyType::USD,
                s => {
                    errors.write_error(
                        format!("Invalid Currency Code: {} specified in CSV header\n", s).as_str(),
                    );
                    CurrencyType::CAD
                }
            }
        }
        Err(s) => {
            errors.write_error(s.as_str());
        }
    }

    match validate_csv_header(&mut rdr, "Payment Date") {
        Ok(s) => {
            csv_header.payment_date = match NaiveDate::parse_from_str(s.as_str(), "%Y/%m/%d") {
                Ok(d) => (d.year() as u64, d.ordinal() as u64),
                Err(s) => {
                    errors.write_error(format!("Could not parse payment date. Date should be in the form of YYYY/MM/DD: {}\n", s.to_string().as_str()).as_str());
                    (0, 0)
                }
            };
        }
        Err(s) => {
            errors.write_error(s.as_str());
        }
    }

    match validate_csv_header(&mut rdr, "Transaction Code") {
        Ok(s) => {
            csv_header.transaction_code = s;
        }
        Err(s) => {
            errors.write_error(s.as_str());
        }
    }

    let mut cpa005_record = CPA005Record::new();

    cpa005_record
        .set_client_number(csv_header.client_number.clone())
        .set_destination_currency_code(csv_header.currency_code)
        .set_file_creation_number(1)
        .set_file_creation_date(2023, 1);

    for rec in rdr.records().skip(1) {
        let mut payment = BasicPayment::new();
        payment.record_type = record_type;

        let rec = match rec {
            Ok(rec) => rec,
            Err(e) => {
                errors.write_error(e.to_string().as_str());
                continue;
            }
        };

        let row: CSVRow = match rec.deserialize(None) {
            Ok(s) => s,
            Err(e) => {
                errors.write_error(e.to_string().as_str());
                continue;
            }
        };

        if row.customer_number.trim().len() == 0 {
            continue;
        }

        if row.suspend.trim().to_ascii_uppercase() == "Y" {
            continue;
        }

        payment.set_client_number(csv_header.client_number.clone());

        let mut payment_segment = BasicPaymentSegment::new();

        payment_segment
            .set_transaction_code(csv_header.transaction_code.clone())
            .set_client_name(csv_header.client_name.clone())
            .set_customer_number(row.customer_number)
            .set_customer_name(row.customer_name)
            .set_financial_institution_number(row.bank)
            .set_financial_institution_branch_number(row.branch)
            .set_account_number(row.account)
            .set_payment_date(csv_header.payment_date.0, csv_header.payment_date.1)
            .set_client_number(csv_header.client_number.clone())
            .set_client_short_name(if csv_header.client_name.len() > 15 {
                csv_header.client_name[0..15].to_string()
            } else {
                csv_header.client_name.to_string()
            })
            .set_amount(match parse_dollar_amount_to_cents(&row.amount) {
                Some(d) => d,
                None => {
                    errors.write_error(
                        format!("Failed to parse payment amount: {}", row.amount).as_str(),
                    );
                    continue;
                }
            });

        payment.error_log.merge_log(&payment_segment.error_log);
        cpa005_record.error_log.merge_log(&payment.error_log);

        payment.segments.push(payment_segment);
        cpa005_record.add_basic_payment(payment);
    }

    errors.merge_log(&cpa005_record.error_log);

    if errors.has_errors() {
        Ok(cpa005_record.build())
    } else {
        Err(errors)
    }
}
