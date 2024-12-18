use super::error::ErrorLog;
use super::types::RecordType;
use super::utils::n_digits;
pub struct BasicPaymentSegment {
    pub transaction_code: String,
    pub amount: u64,
    pub payment_date: (u64, u64),
    pub financial_institution_number: String,
    pub financial_institution_branch_number: String,
    pub account_number: String,
    pub client_short_name: String,
    pub customer_name: String,
    pub client_name: String,
    pub client_number: String,
    pub customer_number: String,
    pub client_sundry_information: String,
    pub error_log: ErrorLog,
}
impl BasicPaymentSegment {
    pub fn new() -> Self {
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
            error_log: ErrorLog::new(),
        }
    }

    pub fn set_transaction_code(&mut self, code: String) -> &mut Self {
        if code.len() != 3 {
            self.error_log.write_error(
                format!(
                    "Transaction code must be 3 digits, received {} instead",
                    self.transaction_code
                )
                .as_str(),
            );
            return self;
        }

        self.transaction_code = code;

        self
    }

    pub fn set_amount(&mut self, cents: u64) -> &mut Self {
        self.amount = cents;

        self
    }

    pub fn set_payment_date(&mut self, year: u64, day: u64) -> &mut Self {
        if day == 0 {
            self.error_log.write_error("Payment Date Day number is 0");
            return self;
        }

        self.payment_date = (year % 100, day);

        self
    }

    pub fn set_financial_institution_number(&mut self, no: String) -> &mut Self {
        self.financial_institution_number = format!("{:0>4}", no);

        self
    }

    pub fn set_financial_institution_branch_number(&mut self, no: String) -> &mut Self {
        if no.parse::<u64>().is_err() {
            self.error_log.write_error("Branch number must be 5 digits");
            return self;
        }

        self.financial_institution_branch_number = format!("{:0>5}", no);

        self
    }

    pub fn set_account_number(&mut self, account_no: String) -> &mut Self {
        for c in account_no.chars() {
            if !c.is_ascii_digit() {
                self.error_log
                    .write_error("Account number must only include digits");
                return self;
            }
        }

        if account_no.len() > 12 {
            self.error_log
                .write_error("Account number cannot exceed 12 digits");
            return self;
        }

        self.account_number = account_no;

        self
    }

    pub fn set_client_short_name(&mut self, short_name: String) -> &mut Self {
        if short_name.len() > 15 {
            self.error_log
                .write_error("Client Short Name must not exceed 15 characters");
            return self;
        }

        self.client_short_name = short_name;

        self
    }

    pub fn set_customer_name(&mut self, customer_name: String) -> &mut Self {
        if customer_name.len() > 30 {
            self.error_log
                .write_error("Customer Name must not exceed 30 characters");
            return self;
        }

        self.customer_name = customer_name;
        self
    }

    pub fn set_client_name(&mut self, client_name: String) -> &mut Self {
        if client_name.len() > 30 {
            self.error_log
                .write_error("Client Name must not exceed 30 characters");
            return self;
        }

        self.client_name = client_name;

        self
    }

    pub fn set_client_number(&mut self, client_number: String) -> &mut Self {
        if client_number.len() != 10 {
            self.error_log
                .write_error("Client number must be exactly 10 numeric digits long");
            return self;
        }

        if client_number.parse::<u64>().is_err() {
            self.error_log
                .write_error("Client number must not contain non-numeric digits");
            return self;
        }

        self.client_number = client_number;

        self
    }

    pub fn set_customer_number(&mut self, customer_number: String) -> &mut Self {
        if customer_number.len() > 19 {
            self.error_log
                .write_error("Customer number must not exceed 19 characters");
            return self;
        }
        self.customer_number = customer_number;

        self
    }

    pub fn set_customer_sundry_information(&mut self, info: String) -> &mut Self {
        if self.client_sundry_information.len() > 15 {
            self.error_log
                .write_error("Client Sundry Information must not exceed 15 characters");
            return self;
        }

        self.client_sundry_information = info;

        self
    }

    pub fn build(&self) -> String {
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

pub struct BasicPayment {
    pub record_type: RecordType,
    pub record_count: u32,
    pub client_number: String,
    pub file_creation_number: u32,
    pub segments: Vec<BasicPaymentSegment>,
    pub error_log: ErrorLog,
}

impl BasicPayment {
    pub fn new() -> Self {
        Self {
            record_type: RecordType::Credit,
            record_count: 0,
            client_number: String::new(),
            file_creation_number: 0,
            segments: Vec::new(),
            error_log: ErrorLog::new(),
        }
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

    pub fn build(&self) -> String {
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
