/*
 * RBC: CSV to CPA-005 Conversion Tool
 *
 * This code implements the following specifications:
 *
 * ACH Direct Payments (PAP-PAD) Service Canadian Payments Association CPA-005 Debit File Format Specifications
 * https://www.rbcroyalbank.com/ach/file-451771.pdf
 *
 * ACH Direct Deposits (PDS) Service Canadian Payments Association CPA-005 Credit File Format Specifications
 * https://www.rbcroyalbank.com/ach/file-451770.pdf
 *
 * Copyright (C) 2023 David Yue
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use chrono::{Datelike, NaiveDate};
use csv::{Reader, ReaderBuilder, StringRecord};
use serde::Deserialize;
use std::env::args;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, Read};
#[derive(Debug)]
enum CurrencyType {
    CAD,
    USD,
}

#[derive(Debug)]
enum ProcessingCentre {
    Halifax,
    Montreal,
    Toronto,
    Regina,
    Winnipeg,
    Calgary,
    Vancouver,
}

#[derive(Clone, Copy)]
enum RecordType {
    Header,
    Credit,
    Debit,
    Trailer,
}

impl Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecordType::Header => write!(f, "{}", 'A'),
            RecordType::Credit => write!(f, "{}", 'C'),
            RecordType::Debit => write!(f, "{}", 'D'),
            RecordType::Trailer => write!(f, "{}", 'Z'),
        }
    }
}

struct BasicPaymentSegment {
    transaction_code: String,
    amount: u64,
    payment_date: (u64, u64),
    financial_institution_number: String,
    financial_institution_branch_number: String,
    account_number: String,
    client_short_name: String,
    customer_name: String,
    client_name: String,
    client_number: String,
    customer_number: String,
    client_sundry_information: String,
}
impl BasicPaymentSegment {
    fn new() -> Self {
        Self {
            transaction_code: String::new(),
            amount: 0,
            payment_date: (0, 0),
            financial_institution_number: String::new(),
            financial_institution_branch_number: String::new(),
            account_number: String::new(),
            client_short_name: String::new(),
            customer_name: String::new(),
            client_name: String::new(),
            client_number: String::new(),
            customer_number: String::new(),
            client_sundry_information: String::new(),
        }
    }

    fn set_transaction_code(&mut self, code: String) -> Result<(), &'static str> {
        if code.len() != 3 {
            return Err("Transaction code must be 3 digits");
        }

        self.transaction_code = code;

        Ok(())
    }

    fn set_amount(&mut self, cents: u64) -> Result<(), &'static str> {
        self.amount = cents;

        Ok(())
    }

    fn set_payment_date(&mut self, year: u64, day: u64) -> Result<(), &'static str> {
        if day == 0 {
            return Err("Payment Date Day number is 0");
        }

        self.payment_date = (year % 100, day);
        Ok(())
    }

    fn set_financial_institution_number(&mut self, no: String) -> Result<(), &'static str> {
        self.financial_institution_number = format!("{:0>4}", no);

        return Ok(());
    }

    fn set_financial_institution_branch_number(&mut self, no: String) -> Result<(), &'static str> {
        if no.parse::<u64>().is_err() {
            return Err("Branch number must be 5 digits");
        }

        self.financial_institution_branch_number = format!("{:0>5}", no);
        return Ok(());
    }

    fn set_account_number(&mut self, account_no: String) -> Result<(), &'static str> {
        for c in account_no.chars() {
            if !c.is_ascii_digit() {
                return Err("Account number must only include digits");
            }
        }

        if account_no.len() > 12 {
            return Err("Account number cannot exceed 12 digits");
        }

        self.account_number = account_no;
        return Ok(());
    }

    fn set_client_short_name(&mut self, short_name: String) -> Result<(), &'static str> {
        if short_name.len() > 15 {
            return Err("Client Short Name must not exceed 15 characters");
        }

        self.client_short_name = short_name;

        Ok(())
    }

    fn set_customer_name(&mut self, customer_name: String) -> Result<(), &'static str> {
        if customer_name.len() > 30 {
            return Err("Customer Name must not exceed 30 characters");
        }

        self.customer_name = customer_name;
        Ok(())
    }

    fn set_client_name(&mut self, client_name: String) -> Result<(), &'static str> {
        if client_name.len() > 30 {
            return Err("Client Name must not exceed 30 characters");
        }
        self.client_name = client_name;
        Ok(())
    }

    fn set_client_number(&mut self, client_number: String) -> Result<(), &'static str> {
        if client_number.len() != 10 {
            return Err("Client number must be exactly 10 numeric digits long");
        }

        if client_number.parse::<u64>().is_err() {
            return Err("Client number must not contain non-numeric digits");
        }

        self.client_number = client_number;

        return Ok(());
    }

    fn set_customer_number(&mut self, customer_number: String) -> Result<(), &'static str> {
        if customer_number.len() > 19 {
            return Err("Customer number must not exceed 19 characters");
        }
        self.customer_number = customer_number;
        Ok(())
    }

    fn set_customer_sundry_information(&mut self, info: String) -> Result<(), &'static str> {
        if self.client_sundry_information.len() > 15 {
            return Err("Client Sundry Information must not exceed 15 characters");
        }

        self.client_sundry_information = info;

        Ok(())
    }

    fn build(&self) -> String {
        let mut payload = String::new();

        // Field 5
        payload.push_str(&self.transaction_code);

        // Field 6
        payload.push_str(format!("{:0>8}{:0>2}", self.amount / 100, self.amount % 100).as_str());

        // Field 7
        payload
            .push_str(format!("0{:0>2}{:0>3}", self.payment_date.0, self.payment_date.1).as_str());

        // Field 8
        payload.push_str(
            format!(
                "{}{}",
                self.financial_institution_number, self.financial_institution_branch_number
            )
            .as_str(),
        );

        // Field 9
        payload.push_str(format!("{:<12}", self.account_number).as_str());

        // Field 10
        payload.push_str("0".repeat(22).as_str());

        // Field 11
        payload.push_str("0".repeat(3).as_str());

        // Field 12
        payload.push_str(format!("{:<15}", self.client_short_name).as_str());

        // Field 13
        payload.push_str(format!("{:<30}", self.customer_name).as_str());

        // Field 14
        payload.push_str(format!("{:<30}", self.client_name).as_str());

        // Field 15
        payload.push_str(format!("{:<10}", self.client_number).as_str());

        // Field 16
        payload.push_str(format!("{:<19}", self.customer_number).as_str());

        // Field 17
        payload.push_str("0".repeat(9).as_str());

        // Field 18
        payload.push_str(" ".repeat(12).as_str());

        // Field 19
        payload.push_str(format!("{:<15}", self.client_sundry_information).as_str());

        // Field 20
        payload.push_str(" ".repeat(22).as_str());

        // Field 21
        payload.push_str(" ".repeat(2).as_str());

        // Field 22
        payload.push_str(" ".repeat(11).as_str());

        return payload;
    }
}

fn n_digits(mut v: u32) -> usize {
    let mut count = 0usize;
    while v != 0 {
        count += 1;

        v /= 10;
    }

    return count;
}
struct BasicPayment {
    record_type: RecordType,
    record_count: u32,
    client_number: String,
    file_creation_number: u32,
    segments: Vec<BasicPaymentSegment>,
}

impl BasicPayment {
    fn new() -> Self {
        Self {
            record_type: RecordType::Credit,
            record_count: 0,
            client_number: String::new(),
            file_creation_number: 0,
            segments: Vec::new(),
        }
    }

    fn set_client_number(&mut self, client_number: String) -> Result<(), &'static str> {
        if client_number.parse::<u64>().is_err() {
            return Err("Client number must be exactly 10 numeric digits long");
        }

        self.client_number = client_number;

        return Ok(());
    }

    fn set_file_creation_number(&mut self, no: u32) -> Result<(), &'static str> {
        if n_digits(no) > 4 {
            return Err("File creation number exceeds 4 digits");
        }

        self.file_creation_number = no;

        return Ok(());
    }

    fn build(&self) -> String {
        let mut payload = String::new();

        payload.push(match self.record_type {
            RecordType::Credit => 'C',
            RecordType::Debit => 'D',
            _ => panic!("Expected record of type CREDIT or DEBIT"),
        });
        payload.push_str(format!("{:0>9}", self.record_count).as_str());
        payload.push_str(&self.client_number);
        payload.push_str(format!("{:<4}", self.file_creation_number).as_str());

        for seg in &self.segments {
            payload.push_str(&seg.build())
        }

        return payload;
    }
}

struct CPA005Record {
    current_record_no: u32,
    client_number: String,
    file_creation_number: u32,
    file_creation_date: (u32, u32),
    rbc_processing_centre: ProcessingCentre,
    destination_currency_code: CurrencyType,
    total_debit_amount: u64,
    total_debit_count: u64,
    total_credit_amount: u64,
    total_credit_count: u64,
    basic_payment: Vec<BasicPayment>,
}

// PDS Format: https://www.rbcroyalbank.com/ach/file-451771.pdf
impl CPA005Record {
    fn new() -> Self {
        Self {
            current_record_no: 1,
            client_number: String::new(),
            file_creation_number: 0,
            file_creation_date: (0, 0),
            destination_currency_code: CurrencyType::CAD,
            rbc_processing_centre: ProcessingCentre::Vancouver,
            total_debit_amount: 0,
            total_debit_count: 0,
            total_credit_amount: 0,
            total_credit_count: 0,
            basic_payment: Vec::new(),
        }
    }

    fn _allocate_record_no(&mut self) -> u32 {
        self.current_record_no += 1;

        return self.current_record_no;
    }

    fn add_basic_payment(&mut self, mut payment: BasicPayment) {
        payment.record_count = self._allocate_record_no();

        match payment.record_type {
            RecordType::Credit => {
                self.total_credit_count += 1;
            }
            RecordType::Debit => {
                self.total_debit_count += 1;
            }
            _ => {
                panic!("Basic Payment Record Type can only be CREDIT or DEBIT!");
            }
        }

        payment.set_file_creation_number(payment.record_count);

        for rec in &payment.segments {
            match payment.record_type {
                RecordType::Credit => {
                    self.total_credit_amount += rec.amount;
                }
                RecordType::Debit => {
                    self.total_debit_amount += rec.amount;
                }
                _ => {
                    panic!("Basic Payment Record Type can only be CREDIT or DEBIT!");
                }
            }
        }

        self.basic_payment.push(payment);
    }

    fn set_client_number(&mut self, client_number: String) -> Result<(), &'static str> {
        if client_number.parse::<u64>().is_err() {
            return Err("Client number must be exactly 10 numeric digits long");
        }

        self.client_number = client_number;

        return Ok(());
    }

    fn set_file_creation_number(&mut self, no: u32) -> Result<(), &'static str> {
        if n_digits(no) > 4 {
            return Err("File creation number exceeds 4 digits");
        }

        self.file_creation_number = no;

        return Ok(());
    }

    fn set_file_creation_date(&mut self, year: u32, day: u32) -> Result<(), &'static str> {
        if n_digits(year) > 4 {
            return Err("File Creation Date: Year number exceeds 4 digits");
        }

        if n_digits(day) > 3 {
            return Err("File Creation Date: Day number exceeds 4 digits");
        }

        self.file_creation_date = (year, day);

        return Ok(());
    }

    fn set_destination_currency_code(&mut self, t: CurrencyType) {
        self.destination_currency_code = t;
    }

    fn build_trailer_record(&self) -> String {
        let mut payload = String::new();
        payload.push_str(format!("{}", RecordType::Trailer).as_str());

        payload.push_str(format!("{:0>9}", self.current_record_no + 1).as_str());
        payload.push_str(format!("{}", self.client_number).as_str());
        payload.push_str(format!("{:<4}", self.file_creation_number).as_str());

        payload.push_str(
            format!(
                "{:0>12}{:0>2}",
                self.total_debit_amount / 100,
                self.total_debit_amount % 100
            )
            .as_str(),
        );
        payload.push_str(format!("{:0>8}", self.total_debit_count).as_str());

        payload.push_str(
            format!(
                "{:0>12}{:0>2}",
                self.total_credit_amount / 100,
                self.total_credit_amount % 100
            )
            .as_str(),
        );
        payload.push_str(format!("{:0>8}", self.total_credit_count).as_str());

        payload.push_str("0".repeat(1396).as_str());

        return payload;
    }

    fn build_header_record(&self) -> String {
        let mut payload = String::new();

        payload.push_str(format!("{}", RecordType::Header).as_str());
        payload.push_str(format!("{:0>9}", 1).as_str());

        payload.push_str(&self.client_number);
        payload.push_str(format!("{:<4}", self.file_creation_number).as_str());
        payload.push_str(
            format!(
                "0{:0>2}{:0>3}",
                self.file_creation_date.0, self.file_creation_date.1
            )
            .as_str(),
        );

        payload.push_str(match self.rbc_processing_centre {
            ProcessingCentre::Halifax => "00330",
            ProcessingCentre::Montreal => "00310",
            ProcessingCentre::Toronto => "00320",
            ProcessingCentre::Regina => "00278",
            ProcessingCentre::Winnipeg => "00370",
            ProcessingCentre::Calgary => "00390",
            ProcessingCentre::Vancouver => "00300",
        });

        payload.push_str(" ".repeat(20).as_str());

        payload.push_str(match self.destination_currency_code {
            CurrencyType::CAD => "CAD",
            CurrencyType::USD => "USD",
        });

        payload.push_str(" ".repeat(1406).as_str());
        return payload;
    }

    fn build(&self) -> String {
        let mut payload = String::new();

        payload.push_str(&self.build_header_record());
        payload.push_str("\n");
        for payment in &self.basic_payment {
            payload.push_str(&payment.build());
            payload.push_str("\n");
        }

        payload.push_str(&&self.build_trailer_record());

        return payload;
    }
}

fn validate_csv_header<'a>(
    rdr: &'a mut Reader<&[u8]>,
    header_name: &str,
) -> Result<String, String> {
    let mut record = StringRecord::new();

    match rdr.read_record(&mut record) {
        Ok(true) => (),
        _ => {
            return Err(format!(
                "Could not record CSV header record {}\n",
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

fn convert_to_cpa005(csv: String, record_type: RecordType) -> Result<String, String> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv.as_bytes());

    let mut csv_header = CSVHeader::new();
    let mut errors = String::new();

    match validate_csv_header(&mut rdr, "Client Name") {
        Ok(s) => {
            csv_header.client_name = s.to_string();
        }
        Err(s) => {
            errors.push_str(s.as_str());
        }
    }

    match validate_csv_header(&mut rdr, "Client Number") {
        Ok(s) => {
            csv_header.client_number = s;
        }
        Err(s) => {
            errors.push_str(s.as_str());
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
                    errors.push_str(
                        format!("Invalid Processing Centre: {} specified in CSV header\n", s)
                            .as_str(),
                    );
                    ProcessingCentre::Vancouver
                }
            }
        }
        Err(s) => {
            errors.push_str(s.as_str());
        }
    }

    match validate_csv_header(&mut rdr, "Currency Code") {
        Ok(s) => {
            csv_header.currency_code = match s.to_uppercase().as_str() {
                "CAD" => CurrencyType::CAD,
                "USD" => CurrencyType::USD,
                s => {
                    errors.push_str(
                        format!("Invalid Currency Code: {} specified in CSV header\n", s).as_str(),
                    );
                    CurrencyType::CAD
                }
            }
        }
        Err(s) => {
            errors.push_str(s.as_str());
        }
    }

    match validate_csv_header(&mut rdr, "Payment Date") {
        Ok(s) => {
            csv_header.payment_date = match NaiveDate::parse_from_str(s.as_str(), "%Y/%m/%d") {
                Ok(d) => (d.year() as u64, d.ordinal() as u64),
                Err(s) => {
                    errors.push_str(format!("Could not parse payment date. Date should be in the form of YYYY/MM/DD: {}\n", s.to_string().as_str()).as_str());
                    (0, 0)
                }
            };
        }
        Err(s) => {
            errors.push_str(s.as_str());
        }
    }

    match validate_csv_header(&mut rdr, "Transaction Code") {
        Ok(s) => {
            csv_header.transaction_code = s;
        }
        Err(s) => {
            errors.push_str(s.as_str());
        }
    }

    let mut cpa005_record = CPA005Record::new();
    
    match cpa005_record.set_client_number(csv_header.client_number.clone()) {
        Ok(()) => (),
        Err(s) => errors.push_str(s),
    }

    cpa005_record.set_destination_currency_code(csv_header.currency_code);

    match cpa005_record.set_file_creation_number(1) {
        Ok(()) => (),
        Err(s) => errors.push_str(s),
    }
    match cpa005_record.set_file_creation_date(2023, 1) {
        Ok(()) => (),
        Err(s) => errors.push_str(s),
    }

    for rec in rdr.records().skip(1) {
        let mut payment = BasicPayment::new();

        let rec = match rec {
            Ok(rec) => rec,
            Err(e) => {
                errors.push_str(e.to_string().as_str());
                continue;
            }
        };

        let row: CSVRow = match rec.deserialize(None) {
            Ok(s) => s,
            Err(e) => {
                errors.push_str(e.to_string().as_str());
                continue;
            }
        };

        if row.suspend.trim().to_ascii_uppercase() == "Y" {
            continue;
        }
        
        payment.record_type = record_type;

        match payment.set_client_number(csv_header.client_number.clone()) {
            Ok(_) => (),
            Err(s) => {
                errors.push_str(s);
                break;
            }
        };

        let mut payment_segment = BasicPaymentSegment::new();

        match payment_segment.set_transaction_code(csv_header.transaction_code.clone()) {
            Ok(_) => (),
            Err(s) => {
                errors.push_str(s);
                break;
            }
        }
        match payment_segment.set_client_name(csv_header.client_name.clone()) {
            Ok(()) => (),
            Err(s) => errors.push_str(s),
        }
        match payment_segment.set_customer_number(row.customer_number) {
            Ok(()) => (),
            Err(s) => errors.push_str(s),
        }
        match payment_segment.set_customer_name(row.customer_name) {
            Ok(()) => (),
            Err(s) => errors.push_str(s),
        }
        match payment_segment.set_financial_institution_number(row.bank) {
            Ok(()) => (),
            Err(s) => errors.push_str(s),
        }
        match payment_segment.set_financial_institution_branch_number(row.branch) {
            Ok(()) => (),
            Err(s) => errors.push_str(s),
        }
        match payment_segment.set_account_number(row.account) {
            Ok(()) => (),
            Err(s) => errors.push_str(s),
        }
        match payment_segment.set_payment_date(csv_header.payment_date.0, csv_header.payment_date.1)
        {
            Ok(()) => (),
            Err(s) => errors.push_str(s),
        }
        match payment_segment.set_client_number(csv_header.client_number.clone()) {
            Ok(()) => (),
            Err(s) => errors.push_str(s),
        }

        let short_name: &str;

        if csv_header.client_name.len() > 15 {
            short_name = &csv_header.client_name[0..15];
        } else {
            short_name = &csv_header.client_name;
        }

        match payment_segment.set_client_short_name(short_name.to_string()) {
            Ok(()) => (),
            Err(s) => errors.push_str(s),
        }

        match parse_dollar_amount_to_cents(&row.amount) {
            Some(d) => match payment_segment.set_amount(d) {
                Ok(()) => (),
                Err(s) => errors.push_str(s),
            },
            None => {
                errors.push_str(format!("Failed to parse payment amount: {}", row.amount).as_str());
                continue;
            }
        };

        payment.segments.push(payment_segment);

        cpa005_record.add_basic_payment(payment);
    }

    if errors.len() == 0 {
        return Ok(cpa005_record.build());
    } else {
        return Err(errors);
    }
}

use actix_multipart::Multipart;
use actix_web::http::header::{ContentDisposition, ContentType};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use futures::{StreamExt, TryStreamExt};

#[derive(Deserialize)]
struct ConvertRequestQuery {
    convtype: String
}

#[post("/convert")]
async fn convert(mut body: Multipart, q: web::Query<ConvertRequestQuery>) -> HttpResponse {
    let mut file_data = String::new();
    let mut file_name = String::new();
    while let Ok(Some(mut p)) = body.try_next().await {
        file_name = p.content_disposition().get_filename().unwrap().to_string();
        while let Some(chunk) = p.next().await {
            let chunk = chunk.unwrap();
            file_data.push_str(&String::from_utf8_lossy(chunk.as_ref()));
        }
    }



    let cpa_format = match q.convtype.trim() {
        "PDS" => convert_to_cpa005(file_data, RecordType::Credit).unwrap(),
        "PAD" => convert_to_cpa005(file_data, RecordType::Debit).unwrap(),
        _ => {
            return HttpResponse::BadRequest().finish();
        }

    };

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(ContentDisposition::attachment(file_name))
        .body(cpa_format)
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body(include_str!("../../ui/dist/index.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(convert))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
