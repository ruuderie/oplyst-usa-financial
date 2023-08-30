

enum BusinessCategory {
    Employment,
    Education,
    RealEstate,
    Mailing,
    Bakery,
    Medical,
    EquipmentRental,
    DataCommunication,
    Storage,
    Development,
    Others,
}

struct Address {
    physical: Option<String>,
    mailing: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zipcode: Option<String>,
    street: Option<String>,
    address_line2: Option<String>,
    postal_code: Option<String>,
    geo_location: Option<String>,
    country: Option<String>,
    continent: Option<String>,
    location_type: Option<String>,
}

struct SocialMedia {
    linkedin_url: Option<String>,
    linkedin_username: Option<String>,
    facebook_url: Option<String>,
    facebook_username: Option<String>,
    twitter_url: Option<String>,
    twitter_username: Option<String>,
    github_url: Option<String>,
    github_username: Option<String>,
    company_linkedin_url: Option<String>,
    company_facebook_url: Option<String>,
    company_twitter_url: Option<String>,
}

struct Company {
    name: Option<String>,
    dba_name: Option<String>,
    industry: Option<BusinessCategory>,
    description: Option<String>,
    website: Option<String>,
    size: Option<String>,
    founded: Option<String>,
    phone: Option<String>,
    fax: Option<String>,
    annual_sales: Option<String>,
    employees: Option<String>,
    sic_code: Option<String>,
    profile: Option<String>,
    services: Option<String>,
    service_description: Option<String>,
    capability: Option<String>,
    categories: Vec<BusinessCategory>,
    credit_score: Option<String>,
    commodity_codes: Option<Vec<String>>,
    exchange: Option<String>,
    ticker_symbol: Option<String>,
}

struct Person {
    firstname: Option<String>,
    middlename: Option<String>,
    middleinitial: Option<String>,
    lastname: Option<String>,
    title: Option<String>,
    email: Option<String>,
    mobile: Option<String>,
    phone: Option<String>,
    skills: Option<String>,
    birth_date: Option<String>,
    gender: Option<String>,
    linkedin_connections: Option<String>,
    inferred_salary: Option<String>,
    years_experience: Option<String>,
    summary: Option<String>,
    contact: Option<String>,
    sub_role: Option<String>,
    countries: Option<Vec<String>>,
    interests: Option<Vec<String>>,
    organization_role: Option<String>,
}

struct Certification {
    owner_ethnicity: Option<String>,
    agency: Option<String>,
    cert_date: Option<String>,
    sbe_certification: Option<String>,
    certification_type: Option<String>,
    expiration: Option<String>,
}

struct Record {
    freight_role: Option<String>,
    address: Address,
    social_media: SocialMedia,
    company: Company,
    person: Person,
    certification: Certification,
    business_description: Option<String>,
    county: Option<String>,
    work_districts_regions: Option<String>,
    suppliers: Option<String>,
}
