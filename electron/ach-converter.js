/**
 * Author: David Yue
 * Email: davidyue5819@gmail.com
 */
const moment = require("moment");
const xl = require("xlsx");
const fs = require("fs");

class Formatter {
    /**
     *
     * @param {string} s
     * @param {number} n
     */
    static alphanumeric(s, n) {
        let w = s.toString();

        return w + " ".repeat(n - w.length);
    }

    /**
     *
     * @param {number} amount
     * @param {number} width
     */
    static money(amount, width) {
        let flt = amount.toFixed(2).toString().replace(".", "");

        return "0".repeat(width - flt.length) + flt;
    }

    /**
     *
     * @param {number} n
     * @param {number} width
     */
    static numeric(n, width) {
        return "0".repeat(width - n.toString().length) + n.toString();
    }
}

class ACHRecord {
    /**
     *
     * @param {string} type
     * @param {number} count
     * @param {string} client_no
     * @param {number} file_no
     */
    constructor(type, count, client_no, file_no) {
        this.type = type;
        this.count = count;
        this.client_no = client_no;
        this.file_no = file_no;
    }
}

class ACHHeader extends ACHRecord {
    constructor(
        count,
        client_no,
        file_no,
        file_date,
        processing_centre,
        currency_code
    ) {
        super("A", count, client_no, file_no);
        this.file_date = file_date;
        this.processing_centre = processing_centre;
        this.currency_code = currency_code;
    }

    toString() {
        return [
            Formatter.alphanumeric(this.type, 1),
            Formatter.numeric(this.count, 9),
            Formatter.alphanumeric(this.client_no, 10),
            Formatter.alphanumeric(this.file_no, 4),
            Formatter.numeric(this.processing_centre, 5),
            " ".repeat(20),
            Formatter.alphanumeric(this.currency_code, 3),
        ].join("");
    }
}

class Segment {
    /**
     *
     * @param {string} transaction_code
     * @param {number} amount
     * @param {moment.Moment} payment_date
     * @param {*} transit
     * @param {*} account_no
     * @param {*} client_short_name
     * @param {*} customer_name
     * @param {*} client_long_name
     * @param {*} client_no
     * @param {*} customer_no
     * @param {*} client_sundry_no
     */
    constructor(
        transaction_code,
        amount,
        payment_date,
        transit,
        account_no,
        client_short_name,
        customer_name,
        client_long_name,
        client_no,
        customer_no,
        client_sundry_no
    ) {
        this.transaction_code = transaction_code;
        this.amount = amount;
        this.payment_date = payment_date;
        this.transit = transit;
        this.account_no = account_no;
        this.client_short_name = client_short_name;
        this.customer_name = customer_name;
        this.client_long_name = client_long_name;
        this.client_no = client_no;
        this.customer_no = customer_no;
        this.client_sundry_no = client_sundry_no;
    }

    toString() {
        return [
            Formatter.alphanumeric(this.transaction_code, 3),
            Formatter.money(this.amount, 10),
            this.payment_date.format("0YYDDDD"),
            Formatter.numeric(this.transit, 9),
            Formatter.alphanumeric(this.account_no, 12),
            "0".repeat(22),
            "0".repeat(3),
            Formatter.alphanumeric(this.client_short_name, 15),
            Formatter.alphanumeric(this.customer_name, 30),
            Formatter.alphanumeric(this.client_long_name, 30),
            Formatter.alphanumeric(this.client_no, 10),
            Formatter.alphanumeric(this.customer_no, 19),
            "0".repeat(9),
            " ".repeat(12),
            Formatter.alphanumeric(this.client_sundry_no, 15),
            " ".repeat(22),
            " ".repeat(2),
            " ".repeat(11),
        ].join("");
    }
}

class ACHBasicPayment extends ACHRecord {
    /**
     *
     * @param {string} type
     * @param {number} count
     * @param {string} client_no
     * @param {number} file_no
     */
    constructor(type, count, client_no, file_no) {
        super(type, count, client_no, file_no);
        this.segments = [];
    }

    /**
     *
     * @param {Segment} s
     */
    add_segment(s) {
        this.segments.push(s);
    }

    toString() {
        return (
            [
                Formatter.alphanumeric(this.type, 1),
                Formatter.numeric(this.count, 9),
                Formatter.alphanumeric(this.client_no, 10),
                Formatter.alphanumeric(this.file_no, 4),
            ].join("") + this.segments.map((s) => s.toString()).join("")
        );
    }
}

class ACHTrailer extends ACHRecord {
    constructor(count, client_no, file_no, debit_amount, debit_count) {
        super("Z", count, client_no, file_no);
        this.debit_amount = debit_amount;
        this.debit_count = debit_count;
    }

    toString() {
        return [
            Formatter.alphanumeric(this.type, 1),
            Formatter.numeric(this.count, 9),
            Formatter.alphanumeric(this.client_no, 10),
            Formatter.alphanumeric(this.file_no, 4),
            Formatter.money(this.debit_amount, 14),
            Formatter.numeric(this.debit_count, 8),
            "0".repeat(4),
            "0".repeat(8),
            "0".repeat(1396),
        ].join("");
    }
}

function writeACHFile(xlFile, outFile, mode) {
    let in_file = xl.readFile(xlFile);

    let first_sheet = in_file.SheetNames[0];
    let worksheet = in_file.Sheets[first_sheet];
    /**
     *
     * @param {number} row
     * @param {number} col
     * @returns {string}
     */
    let xl_row_col = (row, col) => {
        let cols = [
            "A",
            "B",
            "C",
            "D",
            "E",
            "F",
            "G",
            "H",
            "I",
            "J",
            "K",
            "L",
            "M",
            "N",
            "O",
            "P",
            "Q",
            "R",
            "S",
            "T",
            "U",
            "V",
            "W",
            "X",
            "Y",
            "Z",
        ];

        let addr = "";

        col -= 1;

        while (true) {
            addr = cols[col % 27] + addr;
            col = Math.floor(col / 26);

            if (col == 0) break;
        }

        addr += row.toFixed(0).toString();

        return addr;
    };

    /**
     *
     * @param {number} row
     * @param {number} col
     * @returns
     */
    let cell = (row, col) => {
        let c = worksheet[xl_row_col(row, col)];

        if (!c) {
            return null;
        }


        if (typeof c.v == "number") {
            return c.v
        } else {
            return c.v.trim()
        }
    };

    let RECORD_COUNT = 1;
    let client_name = cell(1, 2);
    let client_no = cell(2, 2);
    let processing_centre = cell(3, 2);
    let currency_code = cell(4, 2);
    let payment_date = cell(5, 2);
    let payment_moment = moment(payment_date, "YYYY/MM/DD");
    let transaction_code = cell(6, 2);

    let header = new ACHHeader(
        RECORD_COUNT,
        client_no,
        RECORD_COUNT,
        moment(),
        processing_centre,
        currency_code
    );

    RECORD_COUNT++;

    let payment_records = [];

    let r = 8;

    let total_debit_amount = 0;
    let total_debit_records = 0;

    while (true) {
        let b = new ACHBasicPayment(
            mode,
            RECORD_COUNT,
            client_no,
            RECORD_COUNT
        );

        let customer_no = cell(r, 1);
        let customer_name = cell(r, 2);
        let bank = cell(r, 3);
        let branch = cell(r, 4);
        let account = cell(r, 5);
        let amount = cell(r, 6);
        let suspend = cell(r, 7);
        
        if (!customer_no) {
            break;
        }

        if (suspend == "y") {
            r++;
            continue;
        }

        b.add_segment(
            new Segment(
                transaction_code,
                amount,
                payment_moment,
                Formatter.numeric(bank, 4) + Formatter.numeric(branch, 5),
                account,
                "",
                customer_name,
                client_name,
                client_no,
                customer_no,
                ""
            )
        );

        RECORD_COUNT++;
        r++;
        total_debit_amount += amount;
        total_debit_records++;

        payment_records.push(b);
    }

    let t = new ACHTrailer(
        RECORD_COUNT,
        client_no,
        RECORD_COUNT,
        total_debit_amount,
        total_debit_records
    );

    let fd = fs.openSync(outFile, "w");

    fs.writeSync(fd, header.toString());

    fs.writeSync(fd, "\n\n");

    for (let rec of payment_records) {
        fs.writeSync(fd, rec.toString());
        fs.writeSync(fd, "\n\n");
    }

    fs.writeSync(fd, t.toString());

    fs.closeSync(fd);
}

module.exports = writeACHFile;