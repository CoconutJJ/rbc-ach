use super::error::ErrorLog;
use super::payment::BasicPayment;
use super::types::{CurrencyType, ProcessingCentre, RecordType};
use super::utils::n_digits;
pub struct CPA005Record {
    pub current_record_no: u32,
    pub client_number: String,
    pub file_creation_number: u32,
    pub file_creation_date: (u32, u32),
    pub rbc_processing_centre: ProcessingCentre,
    pub destination_currency_code: CurrencyType,
    pub total_debit_amount: u64,
    pub total_debit_count: u64,
    pub total_credit_amount: u64,
    pub total_credit_count: u64,
    pub basic_payment: Vec<BasicPayment>,
    pub error_log: ErrorLog,
}

// PDS Format: https://www.rbcroyalbank.com/ach/file-451771.pdf
impl CPA005Record {
    pub fn new() -> Self {
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
            error_log: ErrorLog::new(),
        }
    }

    pub fn _allocate_record_no(&mut self) -> u32 {
        self.current_record_no += 1;

        return self.current_record_no;
    }

    pub fn add_basic_payment(&mut self, mut payment: BasicPayment) -> &mut Self {
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

        self
    }

    pub fn set_client_number(&mut self, client_number: String) -> &mut Self {
        if client_number.parse::<u64>().is_err() {
            self.error_log
                .write_error("Client number must be exactly 10 numeric digits long");
            return self;
        }

        self.client_number = client_number;

        self
    }

    pub fn set_file_creation_number(&mut self, no: u32) -> &mut Self {
        if n_digits(no) > 4 {
            self.error_log
                .write_error("File creation number exceeds 4 digits");
            return self;
        }

        self.file_creation_number = no;

        self
    }

    pub fn set_file_creation_date(&mut self, year: u32, day: u32) -> &mut Self {
        if n_digits(year) > 4 {
            self.error_log
                .write_error("File Creation Date: Year number exceeds 4 digits");
            return self;
        }

        if n_digits(day) > 3 {
            self.error_log
                .write_error("File Creation Date: Day number exceeds 4 digits");
            return self;
        }

        self.file_creation_date = (year, day);

        self
    }

    pub fn set_destination_currency_code(&mut self, t: CurrencyType) -> &mut Self {
        self.destination_currency_code = t;
        self
    }

    pub fn build_trailer_record(&self) -> String {
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

    pub fn build_header_record(&self) -> String {
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

    pub fn build(&self) -> String {
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
