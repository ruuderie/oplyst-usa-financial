
#[derive(Clone, Copy)]
enum Period {
    Monthly,
    Annual,
}

struct Tenant {
    annual_rent: f64,
}

struct Expense {
    category: String,
    amount: f64,
    period: Period,
}

impl Expense {
    fn new(category: &str, amount: f64, period: Period) -> Self {
        Self {
            category: category.to_string(),
            amount,
            period,
        }
    }

    fn annualize(&self) -> f64 {
        match self.period {
            Period::Monthly => self.amount * 12.0,
            Period::Annual => self.amount,
        }
    }
}

struct Loan {
    amount: f64,
    rate: f64,
    annual_payment: f64,
}

struct Property {
    tenants: Vec<Tenant>,
    expenses: Vec<Expense>,
    loan: Loan,
    vacancy_rate: f64,
    purchase_price: f64,
    property_value: f64
}

impl Property {
    fn new(vacancy_rate: f64) -> Self {
        Self {
            tenants: Vec::new(),
            expenses: Vec::new(),
            loan: Loan {
                amount: 0.0,
                rate: 0.0,
                annual_payment: 0.0,
            },
            vacancy_rate,
            purchase_price: 0.0,
            property_value: 0.0,
        }
    }
    fn set_purchase_price(&mut self, purchase_price: f64) {
        self.purchase_price = purchase_price;
    }
    fn set_property_value(&mut self, property_value: f64) {
        self.property_value = property_value;
    }

    fn add_tenant(&mut self, tenant: Tenant) {
        self.tenants.push(tenant);
    }

    fn add_expense(&mut self, expense: Expense) {
        self.expenses.push(expense);
    }

    fn set_loan(&mut self, loan: Loan) {
        self.loan = loan;
    }

    fn calculate_total_expenses(&self) -> f64 {
        self.expenses.iter().map(|expense| expense.annualize()).sum()
    }

    fn calculate_noi(&self) -> f64 {
        let gross_income: f64 = self.tenants.iter().map(|tenant| tenant.annual_rent).sum();
        let adjusted_gross_income = gross_income * (1.0 - self.vacancy_rate);
        adjusted_gross_income - self.calculate_total_expenses()
    }

    fn calculate_dscr(&self) -> f64 {
        let noi = self.calculate_noi();
        noi / self.loan.annual_payment
    }
}

fn doPropertyAnalysis(){
    let mut property = Property::new(0.1);
    property.add_tenant(Tenant { annual_rent: 50000.0 });
    property.add_tenant(Tenant { annual_rent: 60000.0 });
    
    property.add_expense(Expense::new("Real Estate Taxes", 5000.0, Period::Annual));
    property.add_expense(Expense::new("Insurance", 3000.0, Period::Annual));
    property.add_expense(Expense::new("Water/Sewer", 2000.0, Period::Annual));
    property.add_expense(Expense::new("Utilities", 1000.0, Period::Annual));
    property.add_expense(Expense::new("Management", 6000.0, Period::Annual));
    property.add_expense(Expense::new("Repairs/Maintenance", 4000.0, Period::Annual));
    property.add_expense(Expense::new("Cap Ex", 3000.0, Period::Annual));
    
    property.set_loan(Loan { amount: 100000.0, rate: 0.1, annual_payment: 11000.0 });
    
    let noi = property.calculate_noi();
    let dscr = property.calculate_dscr();

    println!("Deal Statement for Banker:");
    println!();
    println!("Gross Income:");
    for (i, tenant) in property.tenants.iter().enumerate() {
        println!("    Tenant {}: ${:.2}", i + 1, tenant.annual_rent);
    }
    println!("Vacancy Rate: {:.2}%\n", property.vacancy_rate * 100.0);
    println!("Total Expenses:");
    for expense in &property.expenses {
        println!("    {}: ${:.2} ({})", expense.category, expense.annualize(), match expense.period {
            Period::Monthly => "Monthly",
            Period::Annual => "Annual",
        });
    }
    println!("Total Expenses: ${:.2}\n", property.calculate_total_expenses());
    println!("Loan Details:");
    println!("    Loan Amount: ${:.2}", property.loan.amount);
    println!("    Interest Rate: {:.2}%", property.loan.rate * 100.0);
    println!("    Annual Payment: ${:.2}\n", property.loan.annual_payment);
    println!("Net Operating Income: ${:.2}", noi);
    println!("Debt Service Coverage Ratio: {:.2}", dscr);

}