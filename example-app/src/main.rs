#[allow(dead_code)]
mod generated;
mod models;

use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get database URL from environment
    let database_url = env::var("AUTOMODEL_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:massword@localhost/postgres".to_string());

    // Connect to database
    match connect_to_database(&database_url).await {
        Ok(pool) => {
            println!("✓ Connected to database");
            run_examples(&pool).await?;
        }
        Err(e) => {
            println!("✗ Failed to connect to database: {}", e);
            println!("To run this example:");
            println!("1. Start a PostgreSQL database");
            println!("2. Run the sql queries in the ./migrations to create necessary tables");
            println!("3. Set AUTOMODEL_DATABASE_URL environment variable");
            println!("4. Run: cargo run");
        }
    }

    Ok(())
}

async fn connect_to_database(database_url: &str) -> Result<PgPool, Box<dyn std::error::Error>> {
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}

async fn run_examples(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nRunning example queries...");

    // Test basic queries
    println!("\n=== Testing Basic Queries ===");
    test_basic_queries(pool).await?;

    // Test conditional update - only update fields that are provided
    println!("\n=== Testing Conditional Update ===");
    test_conditional_update(pool).await?;

    // Test conditional update with diff
    println!("\n=== Testing Conditional Update with Diff ===");
    test_conditional_update_diff(pool).await?;

    // Test structured parameters
    println!("\n=== Testing Structured Parameters ===");
    test_structured_parameters(pool).await?;

    // Test struct reuse
    println!("\n=== Testing Struct Reuse ===");
    test_struct_reuse(pool).await?;

    // Test all PostgreSQL types
    println!("\n=== Testing All PostgreSQL Types ===");
    test_all_types(pool).await?;

    // Test composite type (nested row)
    println!("\n=== Testing Composite Type (Nested Row) ===");
    test_nested_row(pool).await?;

    // Test social links with custom type mapping
    println!("\n=== Testing Social Links with Custom Type Mapping ===");
    test_social_links(pool).await?;

    // Test nullable social links
    println!("\n=== Testing Nullable Social Links ===");
    test_nullable_social_links(pool).await?;

    // Test social links with structured parameters
    println!("\n=== Testing Social Links with Structured Parameters ===");
    test_social_links_structured(pool).await?;

    // Test social links with conditional diff
    println!("\n=== Testing Social Links with Conditional Diff ===");
    test_social_links_diff(pool).await?;

    // Test social links with conditional (no diff)
    println!("\n=== Testing Social Links with Conditional ===");
    test_social_links_conditional(pool).await?;

    // Test batch insert with optional social links
    println!("\n=== Testing Batch Insert with Optional Social Links ===");
    test_social_links_batch(pool).await?;

    // ---- jsonb[] column tests (Vec<Option<UserTag>>) ----

    // Test tags (jsonb[] column) — basic set/get
    println!("\n=== Testing Tags (jsonb[] column) ===");
    test_tags(pool).await?;

    // Test tags with structured parameters
    println!("\n=== Testing Tags with Structured Parameters ===");
    test_tags_structured(pool).await?;

    // Test tags with conditional diff
    println!("\n=== Testing Tags with Conditional Diff ===");
    test_tags_diff(pool).await?;

    // Test tags with conditional (no diff)
    println!("\n=== Testing Tags with Conditional ===");
    test_tags_conditional(pool).await?;

    // Test batch insert with tags
    println!("\n=== Testing Batch Insert with Tags ===");
    test_tags_batch(pool).await?;

    // ---- required jsonb[] column tests (Vec<Option<UserTag>>, NOT NULL) ----

    // Test labels (required jsonb[] column) — basic set/get
    println!("\n=== Testing Labels (required jsonb[] column) ===");
    test_labels(pool).await?;

    // Test labels with structured parameters
    println!("\n=== Testing Labels with Structured Parameters ===");
    test_labels_structured(pool).await?;

    // Test labels with conditional diff
    println!("\n=== Testing Labels with Conditional Diff ===");
    test_labels_diff(pool).await?;

    // Test labels with conditional (no diff)
    println!("\n=== Testing Labels with Conditional ===");
    test_labels_conditional(pool).await?;

    // Test batch insert with labels
    println!("\n=== Testing Batch Insert with Labels ===");
    test_labels_batch(pool).await?;

    println!("\nTo see the actual generated code, check src/generated/ directory");
    println!("Functions are organized into modules: admin.rs, setup.rs, users.rs, and mod.rs");
    println!(
        "The code is regenerated automatically when the build runs after queries.yaml changes!"
    );

    Ok(())
}

async fn test_basic_queries(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    // Admin functions
    match generated::admin::get_current_time(pool).await {
        Ok(time) => println!("Current time: {:?}", time),
        Err(e) => println!("Error getting time: {}", e),
    }

    // Setup functions
    match generated::setup::create_users_table(pool).await {
        Ok(_) => println!("Users table created successfully"),
        Err(e) => println!("Error creating table: {}", e),
    }

    // Users functions
    match generated::users::get_all_users(pool).await {
        Ok(users) => println!("All users: {:?}", users),
        Err(e) => println!("Error listing users: {}", e),
    }

    Ok(())
}

async fn test_conditional_update(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating conditional UPDATE with optional parameters...");

    // First, insert a test user with a unique email
    println!("\n1. Inserting a test user...");
    let timestamp = chrono::Utc::now().timestamp();
    let user = generated::users::insert_user(
        pool,
        "John Doe".to_string(),
        format!("john.doe.{}@example.com", timestamp),
        30,
        models::UserProfile {
            bio: Some("Test user for conditional update demo".to_string()),
            avatar_url: None,
            preferences: models::UserPreferences {
                theme: "dark".to_string(),
                language: "en".to_string(),
                notifications_enabled: true,
            },
            social_links: vec![],
        },
    )
    .await?;
    println!(
        "✓ Created user: ID={}, name={}, email={}, age={:?}",
        user.id, user.name, user.email, user.age
    );

    // Example 1: Update only the name
    println!("\n2. Updating only the name (email and age remain unchanged)...");
    let updated = generated::users::update_user_fields(
        pool,
        Some("Jane Doe".to_string()),
        None, // email not updated
        None, // age not updated
        user.id,
    )
    .await?;
    println!(
        "✓ After updating name: ID={}, name={}, email={}, age={:?}",
        updated.id, updated.name, updated.email, updated.age
    );

    // Example 2: Update only the age
    println!("\n3. Updating only the age (name and email remain unchanged)...");
    let updated = generated::users::update_user_fields(
        pool,
        None, // name not updated
        None, // email not updated
        Some(35),
        user.id,
    )
    .await?;
    println!(
        "✓ After updating age: ID={}, name={}, email={}, age={:?}",
        updated.id, updated.name, updated.email, updated.age
    );

    // Example 3: Update multiple fields at once
    println!("\n4. Updating both name and email (age remains unchanged)...");
    let unique_email = format!("jane.smith.{}@example.com", timestamp);
    let updated = generated::users::update_user_fields(
        pool,
        Some("Jane Smith".to_string()),
        Some(unique_email),
        None, // age not updated
        user.id,
    )
    .await?;
    println!(
        "✓ After updating name and email: ID={}, name={}, email={}, age={:?}",
        updated.id, updated.name, updated.email, updated.age
    );

    // Example 4: Update all fields
    println!("\n5. Updating all fields at once...");
    let unique_email2 = format!("janet.williams.{}@example.com", timestamp);
    let updated = generated::users::update_user_fields(
        pool,
        Some("Janet Williams".to_string()),
        Some(unique_email2),
        Some(40),
        user.id,
    )
    .await?;
    println!(
        "✓ After updating all fields: ID={}, name={}, email={}, age={:?}",
        updated.id, updated.name, updated.email, updated.age
    );

    println!("\n✓ Conditional update examples completed successfully!");
    println!("The conditional syntax $[, field = ${{param?}}] only includes the SET clause when the parameter is Some(value)");

    Ok(())
}

async fn test_conditional_update_diff(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating conditional UPDATE with diff-based comparison...");

    // First, insert a test user with a unique email
    println!("\n1. Inserting a test user...");
    let timestamp = chrono::Utc::now().timestamp();
    let email = format!("alice.cooper.{}@example.com", timestamp);
    let user = generated::users::insert_user(
        pool,
        "Alice Cooper".to_string(),
        email.clone(),
        28,
        models::UserProfile {
            bio: Some("Test user for diff-based conditional update demo".to_string()),
            avatar_url: None,
            preferences: models::UserPreferences {
                theme: "light".to_string(),
                language: "en".to_string(),
                notifications_enabled: true,
            },
            social_links: vec![],
        },
    )
    .await?;
    println!(
        "✓ Created user: ID={}, name={}, email={}, age={:?}",
        user.id, user.name, user.email, user.age
    );

    // Example 1: Update only the name (by passing different old/new values)
    println!("\n2. Updating only the name using diff (email and age stay the same)...");
    let old = generated::users::UpdateUserFieldsDiffParams {
        name: "Alice Cooper".to_string(),
        email: email.clone(),
        age: 28,
    };
    let new = generated::users::UpdateUserFieldsDiffParams {
        name: "Alice Smith".to_string(), // Changed
        email: email.clone(),            // Same
        age: 28,                         // Same
    };
    let updated = generated::users::update_user_fields_diff(pool, &old, &new, user.id).await?;
    println!(
        "✓ After updating name: ID={}, name={}, email={}, age={:?}",
        updated.id, updated.name, updated.email, updated.age
    );

    // Example 2: Update only the age
    println!("\n3. Updating only the age using diff (name and email stay the same)...");
    let old = generated::users::UpdateUserFieldsDiffParams {
        name: "Alice Smith".to_string(),
        email: email.clone(),
        age: 28,
    };
    let new = generated::users::UpdateUserFieldsDiffParams {
        name: "Alice Smith".to_string(), // Same
        email: email.clone(),            // Same
        age: 29,                         // Changed
    };
    let updated = generated::users::update_user_fields_diff(pool, &old, &new, user.id).await?;
    println!(
        "✓ After updating age: ID={}, name={}, email={}, age={:?}",
        updated.id, updated.name, updated.email, updated.age
    );

    // Example 3: Update multiple fields
    println!("\n4. Updating both name and email using diff (age stays the same)...");
    let email2 = format!("alicia.smith.{}@example.com", timestamp);
    let old = generated::users::UpdateUserFieldsDiffParams {
        name: "Alice Smith".to_string(),
        email: email.clone(),
        age: 29,
    };
    let new = generated::users::UpdateUserFieldsDiffParams {
        name: "Alicia Smith".to_string(), // Changed
        email: email2.clone(),            // Changed
        age: 29,                          // Same
    };
    let updated = generated::users::update_user_fields_diff(pool, &old, &new, user.id).await?;
    println!(
        "✓ After updating name and email: ID={}, name={}, email={}, age={:?}",
        updated.id, updated.name, updated.email, updated.age
    );

    // Example 4: Update all fields
    println!("\n5. Updating all fields using diff...");
    let email3 = format!("alicia.johnson.{}@example.com", timestamp);
    let old = generated::users::UpdateUserFieldsDiffParams {
        name: "Alicia Smith".to_string(),
        email: email2.clone(),
        age: 29,
    };
    let new = generated::users::UpdateUserFieldsDiffParams {
        name: "Alicia Johnson".to_string(), // Changed
        email: email3,                      // Changed
        age: 30,                            // Changed
    };
    let updated = generated::users::update_user_fields_diff(pool, &old, &new, user.id).await?;
    println!(
        "✓ After updating all fields: ID={}, name={}, email={}, age={:?}",
        updated.id, updated.name, updated.email, updated.age
    );

    println!("\n✓ Diff-based conditional update examples completed successfully!");
    println!("With conditions_type: true, the function compares old.field != new.field to decide which SET clauses to include");

    Ok(())
}

async fn test_structured_parameters(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating structured parameters...");

    // Create a params struct and insert a user
    println!("\n1. Inserting a user using structured parameters...");
    let timestamp = chrono::Utc::now().timestamp();
    let params = generated::users::InsertUserStructuredParams {
        name: "Bob Builder".to_string(),
        email: format!("bob.builder.{}@example.com", timestamp),
        age: 42,
    };

    let user = generated::users::insert_user_structured(pool, &params).await?;
    println!(
        "✓ Created user: ID={}, name={}, email={}, age={:?}",
        user.id, user.name, user.email, user.age
    );

    // Demonstrate reusing the struct
    println!("\n2. Inserting another user with modified params...");
    let params2 = generated::users::InsertUserStructuredParams {
        name: "Alice Builder".to_string(),
        email: format!("alice.builder.{}@example.com", timestamp),
        age: 38,
    };

    let user2 = generated::users::insert_user_structured(pool, &params2).await?;
    println!(
        "✓ Created user: ID={}, name={}, email={}, age={:?}",
        user2.id, user2.name, user2.email, user2.age
    );

    println!("\n✓ Structured parameters examples completed successfully!");
    println!("With parameters_type: true, all query parameters are passed as a single struct instead of individual parameters");

    Ok(())
}

async fn test_struct_reuse(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating struct reuse across queries...");
    println!("This feature allows queries to reuse structs defined by previous queries,");
    println!("eliminating code duplication when queries share the same parameter structure.\n");

    let timestamp = chrono::Utc::now().timestamp();

    // Example 1: Reusing a Params struct
    println!("1. Query with parameters_type: true generates GetUserByIdAndEmailParams");
    let email = format!("struct.reuse.{}@example.com", timestamp);

    // First insert a test user
    let user = generated::users::insert_user(
        pool,
        "Struct Reuse Test".to_string(),
        email.clone(),
        35,
        models::UserProfile {
            bio: Some("Test user for struct reuse demo".to_string()),
            avatar_url: None,
            preferences: models::UserPreferences {
                theme: "light".to_string(),
                language: "en".to_string(),
                notifications_enabled: true,
            },
            social_links: vec![],
        },
    )
    .await?;
    println!(
        "   ✓ Inserted test user: ID={}, email={}",
        user.id, user.email
    );

    // Use get_user_by_id_and_email which generates GetUserByIdAndEmailParams
    let params = generated::users::GetUserByIdAndEmailParams {
        id: user.id,
        email: email.clone(),
    };

    match generated::users::get_user_by_id_and_email(pool, &params).await? {
        Some(found) => println!(
            "   ✓ Found user: ID={}, name={}, email={}",
            found.id, found.name, found.email
        ),
        None => println!("   ✗ User not found"),
    }

    // Example 2: Reusing the same Params struct in delete query
    println!("\n2. Another query reuses GetUserByIdAndEmailParams (parameters_type: \"GetUserByIdAndEmailParams\")");
    println!("   No new struct is generated - it reuses the existing one!");

    let deleted = generated::users::delete_user_by_id_and_email(pool, &params).await?;
    println!(
        "   ✓ Deleted user: ID={}, email={}",
        deleted.id, deleted.email
    );

    // Example 3: Reusing a return type struct (Item) as params
    println!("\n3. Reusing a return type struct (GetUserByIdAndEmailItem) as params");

    // Insert another test user
    let email2 = format!("struct.item.reuse.{}@example.com", timestamp);
    let user2 = generated::users::insert_user(
        pool,
        "Item Struct Reuse".to_string(),
        email2.clone(),
        40,
        models::UserProfile {
            bio: Some("Test user for item struct reuse demo".to_string()),
            avatar_url: None,
            preferences: models::UserPreferences {
                theme: "dark".to_string(),
                language: "en".to_string(),
                notifications_enabled: false,
            },
            social_links: vec![],
        },
    )
    .await?;
    println!(
        "   ✓ Inserted test user: ID={}, email={}",
        user2.id, user2.email
    );

    // Get the user (returns GetUserByIdAndEmailItem with id, name, email fields)
    let get_params = generated::users::GetUserByIdAndEmailParams {
        id: user2.id,
        email: email2.clone(),
    };

    let user_item = generated::users::get_user_by_id_and_email(pool, &get_params)
        .await?
        .expect("User should exist");
    println!(
        "   ✓ Retrieved user item: ID={}, name={}, email={}",
        user_item.id, user_item.name, user_item.email
    );

    // Update using the same struct type (GetUserByIdAndEmailItem)
    // This demonstrates reusing a return type struct as input params
    let update_params = generated::users::GetUserByIdAndEmailItem {
        id: user_item.id,
        name: "Updated Name".to_string(),
        email: user_item.email.clone(),
    };

    let updated = generated::users::update_user_contact_info(pool, &update_params).await?;
    println!(
        "   ✓ Updated user: ID={}, name={}, email={}",
        updated.id, updated.name, updated.email
    );

    println!("\n✓ Struct reuse examples completed successfully!");
    println!("\nKey benefits:");
    println!("  • Reduces code duplication - one struct definition serves multiple queries");
    println!("  • Type safety - compiler ensures fields match across queries");
    println!("  • Can reuse both Params structs and Item (return type) structs");
    println!("  • Validation at build time ensures referenced structs exist and match");

    Ok(())
}

async fn test_all_types(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
    use rust_decimal::Decimal;
    use sqlx::postgres::types::{PgInterval, PgRange, PgTimeTz};
    use std::str::FromStr;
    use uuid::Uuid;

    println!("Inserting test row with all PostgreSQL types...");

    // Prepare test data
    let bool_col = true;
    let char_col = "A".to_string(); // CHAR(1) in PostgreSQL
    let int2_col: i16 = 32767;
    let int4_col: i32 = 2147483647;
    let int8_col: i64 = 9223372036854775807;
    let float4_col: f32 = 3.14159;
    let float8_col: f64 = 2.718281828459045;
    let numeric_col = Decimal::from_str("12345.67")?;

    let name_col = "test_name".to_string();
    let text_col = "This is a test text".to_string();
    let varchar_col = "varchar test".to_string();
    let bpchar_col = "bpchar    ".to_string(); // Will be padded to 10 chars

    let bytea_col = vec![0xDE, 0xAD, 0xBE, 0xEF];
    // Bit types using bit_vec::BitVec
    let mut bit_col = bit_vec::BitVec::from_elem(8, false);
    bit_col.set(0, true);
    bit_col.set(2, true);
    bit_col.set(4, true);
    bit_col.set(6, true);

    let mut varbit_col = bit_vec::BitVec::from_elem(16, false);
    varbit_col.set(0, true);
    varbit_col.set(2, true);
    varbit_col.set(4, true);
    varbit_col.set(6, true);
    varbit_col.set(8, true);
    varbit_col.set(10, true);
    varbit_col.set(12, true);
    varbit_col.set(14, true);

    let date_col = NaiveDate::from_ymd_opt(2025, 11, 20).unwrap();
    let time_col = NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let timestamp_col = NaiveDateTime::parse_from_str("2025-11-20 14:30:00", "%Y-%m-%d %H:%M:%S")?;
    let timestamptz_col = Utc.with_ymd_and_hms(2025, 11, 20, 14, 30, 0).unwrap();
    let interval_col = PgInterval {
        months: 0,
        days: 1,
        microseconds: (2 * 3600 + 30 * 60) * 1_000_000,
    };
    let timetz_col = PgTimeTz {
        time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
        offset: chrono::FixedOffset::east_opt(0).unwrap(),
    };

    // Range types
    let int4_range_col =
        PgRange::from((std::ops::Bound::Included(1), std::ops::Bound::Excluded(10)));
    let int8_range_col = PgRange::from((
        std::ops::Bound::Included(100i64),
        std::ops::Bound::Included(200i64),
    ));
    let num_range_col = PgRange::from((
        std::ops::Bound::Included(Decimal::from_str("0.5")?),
        std::ops::Bound::Included(Decimal::from_str("99.9")?),
    ));
    let ts_range_col = PgRange::from((
        std::ops::Bound::Included(
            NaiveDate::from_ymd_opt(2025, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        ),
        std::ops::Bound::Included(
            NaiveDate::from_ymd_opt(2025, 12, 31)
                .unwrap()
                .and_hms_opt(23, 59, 59)
                .unwrap(),
        ),
    ));
    let tstz_range_col = PgRange::from((
        std::ops::Bound::Included(Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap()),
        std::ops::Bound::Included(Utc.with_ymd_and_hms(2025, 12, 31, 23, 59, 59).unwrap()),
    ));
    let date_range_col = PgRange::from((
        std::ops::Bound::Included(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
        std::ops::Bound::Included(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()),
    ));

    // Network types - using std::net::IpAddr as sqlx maps INET/CIDR to IpAddr with ipnet feature
    let inet_col: std::net::IpAddr = "192.168.1.1".parse()?;
    let cidr_col: std::net::IpAddr = "192.168.1.0".parse()?;
    let macaddr_col = mac_address::MacAddress::new([0x08, 0x00, 0x2b, 0x01, 0x02, 0x03]);

    // JSON types
    let json_col = serde_json::json!({"key": "value", "number": 42});
    let jsonb_col = serde_json::json!({"name": "test", "tags": ["tag1", "tag2"]});

    // UUID
    let uuid_col = Uuid::parse_str("a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11")?;

    // Array types
    let bool_array_col = vec![true, false, true];
    let int4_array_col = vec![1, 2, 3, 4, 5];
    let int8_array_col = vec![100i64, 200i64, 300i64];
    let text_array_col = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    let float8_array_col = vec![1.1, 2.2, 3.3];

    // Range array types
    let int4_range_array_col = vec![
        PgRange::from((std::ops::Bound::Included(1), std::ops::Bound::Excluded(5))),
        PgRange::from((std::ops::Bound::Included(10), std::ops::Bound::Excluded(20))),
    ];
    let date_range_array_col = vec![
        PgRange::from((
            std::ops::Bound::Included(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
            std::ops::Bound::Included(NaiveDate::from_ymd_opt(2025, 1, 31).unwrap()),
        )),
        PgRange::from((
            std::ops::Bound::Included(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap()),
            std::ops::Bound::Included(NaiveDate::from_ymd_opt(2025, 6, 30).unwrap()),
        )),
    ];

    // Insert the test row
    let result = generated::admin::insert_all_types_test(
        pool,
        bool_col,
        char_col.clone(),
        int2_col,
        int4_col,
        int8_col,
        float4_col,
        float8_col,
        numeric_col,
        name_col.clone(),
        text_col.clone(),
        varchar_col.clone(),
        bpchar_col.clone(),
        bytea_col.clone(),
        bit_col.clone(),
        varbit_col.clone(),
        date_col,
        time_col,
        timestamp_col,
        timestamptz_col,
        interval_col,
        timetz_col,
        int4_range_col,
        int8_range_col,
        num_range_col,
        ts_range_col,
        tstz_range_col,
        date_range_col,
        inet_col,
        cidr_col,
        macaddr_col,
        json_col.clone(),
        jsonb_col.clone(),
        uuid_col,
        bool_array_col.clone(),
        int4_array_col.clone(),
        int8_array_col.clone(),
        text_array_col.clone(),
        float8_array_col.clone(),
        int4_range_array_col.clone(),
        date_range_array_col.clone(),
    )
    .await?;

    println!("✓ Inserted row with ID: {}", result);

    // Read back the inserted row
    println!("Reading back the inserted row...");
    let retrieved = generated::admin::get_all_types_test(pool, result).await?;

    println!("✓ Successfully retrieved row:");
    println!("  ID: {:?}", retrieved.id);
    println!("  Boolean: {:?}", retrieved.bool_col);
    println!("  Int4: {:?}", retrieved.int4_col);
    println!("  Int8: {:?}", retrieved.int8_col);
    println!("  Float8: {:?}", retrieved.float8_col);
    println!("  Text: {:?}", retrieved.text_col);
    println!("  UUID: {:?}", retrieved.uuid_col);
    println!("  JSON: {:?}", retrieved.jsonb_col);
    println!("  Date: {:?}", retrieved.date_col);
    println!("  Timestamp: {:?}", retrieved.timestamp_col);
    println!("  Int4 Range: {:?}", retrieved.int4_range_col);
    println!("  Date Range: {:?}", retrieved.date_range_col);
    println!("  Int4 Array: {:?}", retrieved.int4_array_col);
    println!("  Text Array: {:?}", retrieved.text_array_col);
    println!("  Int4 Range Array: {:?}", retrieved.int4_range_array_col);
    println!("  Date Range Array: {:?}", retrieved.date_range_array_col);

    println!("\n✓ All PostgreSQL types test completed successfully!");

    Ok(())
}

async fn test_nested_row(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing composite type (table row as nested data)...");

    // Get first user or skip test if no users exist
    let users = generated::users::get_all_users(pool).await?;
    if users.is_empty() {
        println!("⚠ No users found, skipping composite type test");
        return Ok(());
    }

    let user_id = users[0].id;

    println!("\n1. Fetching user with composite type (nested row)...");
    let result = generated::users::test_nested_row(pool, user_id).await?;

    println!("✓ Successfully retrieved nested row data:");
    println!("  ID: {}", result.id);
    println!("  Name: {}", result.name);

    if let Some(user_details) = &result.user_details {
        println!("  User Details (composite type):");
        println!("    ID: {}", user_details.id);
        println!("    Name: {}", user_details.name);
        println!("    Email: {}", user_details.email);
        println!("    Status: {:?}", user_details.status);
        println!("    Active: {:?}", user_details.is_active);
        println!("    Age: {:?}", user_details.age);
        println!("    Created: {:?}", user_details.created_at);
    } else {
        println!("  User Details: None");
    }

    println!("\n✓ Composite type (nested row) test completed successfully!");

    Ok(())
}

async fn test_social_links(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::UserSocialLink;

    println!("Testing social links with custom type mapping...");

    let timestamp = chrono::Utc::now().timestamp();

    // 1. Insert a new user with social links
    println!("\n1. Creating a new user with social links...");
    let social_links = vec![
        UserSocialLink {
            name: "GitHub".to_string(),
            url: "https://github.com/testuser".to_string(),
        },
        UserSocialLink {
            name: "Twitter".to_string(),
            url: "https://twitter.com/testuser".to_string(),
        },
    ];

    let new_user = generated::users::insert_user_with_social_links(
        pool,
        "Test User".to_string(),
        format!("test.social.{}@example.com", timestamp),
        social_links.clone(),
    )
    .await?;

    println!("✓ Created user with ID: {}", new_user.id);
    println!("  Name: {}", new_user.name);
    println!("  Email: {}", new_user.email);
    if let Some(links) = &new_user.social_links {
        println!("  Social Links: {} items", links.len());
        for link in links {
            println!("    - {}: {}", link.name, link.url);
        }
    } else {
        println!("  Social Links: None");
    }

    // 2. Retrieve the user's social links
    println!("\n2. Retrieving user's social links...");
    let user_data = generated::users::get_user_social_links(pool, new_user.id).await?;

    println!("✓ Retrieved user data:");
    println!("  ID: {}", user_data.id);
    println!("  Name: {}", user_data.name);
    println!("  Email: {}", user_data.email);
    if let Some(links) = &user_data.social_links {
        println!("  Social Links: {} items", links.len());
        for link in links {
            println!("    - {}: {}", link.name, link.url);
        }
    } else {
        println!("  Social Links: None");
    }

    // 3. Update the user's social links
    println!("\n3. Updating user's social links...");
    let updated_links = vec![
        UserSocialLink {
            name: "GitHub".to_string(),
            url: "https://github.com/newusername".to_string(),
        },
        UserSocialLink {
            name: "LinkedIn".to_string(),
            url: "https://linkedin.com/in/testuser".to_string(),
        },
        UserSocialLink {
            name: "Website".to_string(),
            url: "https://testuser.com".to_string(),
        },
    ];

    let updated_user =
        generated::users::update_user_social_links(pool, updated_links.clone(), new_user.id)
            .await?;

    println!("✓ Updated user social links:");
    println!("  ID: {}", updated_user.id);
    println!("  Name: {}", updated_user.name);
    if let Some(links) = &updated_user.social_links {
        println!("  Social Links: {} items", links.len());
        for link in links {
            println!("    - {}: {}", link.name, link.url);
        }
    }

    // 4. Verify the update by retrieving again
    println!("\n4. Verifying the update...");
    let verified_user = generated::users::get_user_social_links(pool, new_user.id).await?;

    println!("✓ Verified updated social links:");
    if let Some(links) = &verified_user.social_links {
        println!("  Social Links: {} items", links.len());
        for link in links {
            println!("    - {}: {}", link.name, link.url);
        }
    }

    // 5. Test with empty social links
    println!("\n5. Testing with empty social links...");
    let empty_links: Vec<UserSocialLink> = vec![];
    let cleared_user =
        generated::users::update_user_social_links(pool, empty_links, new_user.id).await?;

    println!("✓ Cleared social links:");
    println!("  ID: {}", cleared_user.id);
    if let Some(links) = &cleared_user.social_links {
        println!("  Social Links: {} items (should be 0)", links.len());
    } else {
        println!("  Social Links: 0 items (empty)");
    }

    println!("\n✓ Social links with custom type mapping test completed successfully!");
    println!(
        "  - Custom type Vec<crate::models::UserSocialLink> is properly serialized/deserialized"
    );
    println!("  - JSONB column automatically handles the JSON conversion");
    println!("  - Type safety is maintained throughout insert, update, and query operations");

    Ok(())
}

async fn test_nullable_social_links(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::UserSocialLink;

    println!("Testing nullable social links (Option<Vec<UserSocialLink>>)...");

    let timestamp = chrono::Utc::now().timestamp();

    // 1. Insert a user with social links
    println!("\n1. Creating user with social links...");
    let links = vec![UserSocialLink {
        name: "GitHub".to_string(),
        url: "https://github.com/nulltest".to_string(),
    }];
    let user = generated::users::insert_user_with_social_links(
        pool,
        "Nullable Test".to_string(),
        format!("nullable.social.{}@example.com", timestamp),
        links.clone(),
    )
    .await?;
    println!(
        "✓ Created user ID: {}, social_links: {:?}",
        user.id, user.social_links
    );

    // 2. Set social_links to NULL using nullable update
    println!("\n2. Setting social_links to NULL...");
    let nulled =
        generated::users_array_fields::update_user_social_links_nullable(pool, None, user.id)
            .await?;
    println!(
        "✓ After setting NULL: social_links = {:?}",
        nulled.social_links
    );
    assert!(
        nulled.social_links.is_none(),
        "Expected None after setting NULL"
    );

    // 3. Read back and verify NULL
    println!("\n3. Reading back to verify NULL...");
    let read_back = generated::users::get_user_social_links(pool, user.id).await?;
    println!("✓ Read back: social_links = {:?}", read_back.social_links);
    assert!(
        read_back.social_links.is_none(),
        "Expected None on read back"
    );

    // 4. Set social_links back to Some(vec)
    println!("\n4. Setting social_links back to Some(vec)...");
    let restored_links = vec![
        UserSocialLink {
            name: "Twitter".to_string(),
            url: "https://twitter.com/restored".to_string(),
        },
        UserSocialLink {
            name: "Website".to_string(),
            url: "https://restored.example.com".to_string(),
        },
    ];
    let restored = generated::users_array_fields::update_user_social_links_nullable(
        pool,
        Some(restored_links.clone()),
        user.id,
    )
    .await?;
    println!("✓ Restored: social_links = {:?}", restored.social_links);
    assert!(
        restored.social_links.is_some(),
        "Expected Some after restore"
    );
    assert_eq!(restored.social_links.as_ref().unwrap().len(), 2);

    // 5. Verify restored value
    println!("\n5. Verifying restored value...");
    let verified = generated::users::get_user_social_links(pool, user.id).await?;
    let verified_links = verified.social_links.expect("Expected Some");
    assert_eq!(verified_links.len(), 2);
    println!("✓ Verified: {} links", verified_links.len());
    for link in &verified_links {
        println!("    - {}: {}", link.name, link.url);
    }

    println!("\n✓ Nullable social links test completed!");
    println!("  - Option<Vec<UserSocialLink>> correctly handles NULL ↔ Some transitions");
    println!("  - JSONB column accepts both NULL and valid JSON arrays");

    Ok(())
}

/// Test: parameters_type with Vec<CustomStruct> JSONB field
async fn test_social_links_structured(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::UserSocialLink;

    println!("Testing parameters_type with JSONB custom type...");
    let timestamp = chrono::Utc::now().timestamp();

    let params = generated::users_array_fields::InsertUserSocialLinksStructuredParams {
        name: "Structured Test".to_string(),
        email: format!("structured.social.{}@example.com", timestamp),
        social_links: vec![
            UserSocialLink {
                name: "GitHub".to_string(),
                url: "https://github.com/structured".to_string(),
            },
            UserSocialLink {
                name: "Blog".to_string(),
                url: "https://blog.structured.dev".to_string(),
            },
        ],
    };

    let result =
        generated::users_array_fields::insert_user_social_links_structured(pool, &params).await?;
    println!("✓ Inserted user ID: {}", result.id);
    let links = result.social_links.expect("Expected Some social_links");
    assert_eq!(links.len(), 2);
    assert_eq!(links[0].name, "GitHub");
    assert_eq!(links[1].name, "Blog");
    println!("✓ Social links correctly round-tripped through parameters_type struct");

    println!("\n✓ Structured parameters with JSONB test completed!");
    Ok(())
}

/// Test: conditions_type (diff) with Vec<CustomStruct> JSONB field
async fn test_social_links_diff(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::UserSocialLink;

    println!("Testing conditions_type (diff) with JSONB custom type...");
    let timestamp = chrono::Utc::now().timestamp();

    // Create a user first
    let user = generated::users::insert_user_with_social_links(
        pool,
        "Diff Test".to_string(),
        format!("diff.social.{}@example.com", timestamp),
        vec![UserSocialLink {
            name: "GitHub".to_string(),
            url: "https://github.com/difftest".to_string(),
        }],
    )
    .await?;
    println!("✓ Created user ID: {}", user.id);

    let old = generated::users_array_fields::UpdateUserSocialLinksDiffParams {
        name: "Diff Test".to_string(),
        social_links: vec![UserSocialLink {
            name: "GitHub".to_string(),
            url: "https://github.com/difftest".to_string(),
        }],
    };

    // 1. Change only social_links (name stays the same)
    let new_links = vec![
        UserSocialLink {
            name: "Twitter".to_string(),
            url: "https://twitter.com/difftest".to_string(),
        },
        UserSocialLink {
            name: "Website".to_string(),
            url: "https://difftest.dev".to_string(),
        },
    ];
    let new = generated::users_array_fields::UpdateUserSocialLinksDiffParams {
        name: "Diff Test".to_string(),
        social_links: new_links.clone(),
    };

    let updated =
        generated::users_array_fields::update_user_social_links_diff(pool, &old, &new, user.id)
            .await?;
    let links = updated.social_links.expect("Expected Some");
    assert_eq!(links.len(), 2);
    assert_eq!(links[0].name, "Twitter");
    println!("✓ Diff update changed social_links only (name unchanged)");

    // 2. Change both name and social_links
    let old2 = new.clone();
    let new2 = generated::users_array_fields::UpdateUserSocialLinksDiffParams {
        name: "Diff Test Updated".to_string(),
        social_links: vec![UserSocialLink {
            name: "LinkedIn".to_string(),
            url: "https://linkedin.com/in/difftest".to_string(),
        }],
    };

    let updated2 =
        generated::users_array_fields::update_user_social_links_diff(pool, &old2, &new2, user.id)
            .await?;
    assert_eq!(updated2.name, "Diff Test Updated");
    let links2 = updated2.social_links.expect("Expected Some");
    assert_eq!(links2.len(), 1);
    assert_eq!(links2[0].name, "LinkedIn");
    println!("✓ Diff update changed both name and social_links");

    // 3. No changes (old == new) — should still return current data
    let updated3 =
        generated::users_array_fields::update_user_social_links_diff(pool, &new2, &new2, user.id)
            .await?;
    assert_eq!(updated3.name, "Diff Test Updated");
    println!("✓ Diff with no changes returned current data");

    println!("\n✓ Conditional diff with JSONB test completed!");
    Ok(())
}

/// Test: conditional (no struct) with Option<Vec<CustomStruct>> JSONB field
async fn test_social_links_conditional(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::UserSocialLink;

    println!("Testing conditional (non-diff) with optional JSONB custom type...");
    let timestamp = chrono::Utc::now().timestamp();

    // Create a user first
    let user = generated::users::insert_user_with_social_links(
        pool,
        "Conditional Test".to_string(),
        format!("conditional.social.{}@example.com", timestamp),
        vec![UserSocialLink {
            name: "GitHub".to_string(),
            url: "https://github.com/condtest".to_string(),
        }],
    )
    .await?;
    println!("✓ Created user ID: {}", user.id);

    // 1. Update only social_links (name = None → skip)
    let new_links = vec![UserSocialLink {
        name: "Twitter".to_string(),
        url: "https://twitter.com/condtest".to_string(),
    }];
    let updated = generated::users_array_fields::update_user_social_links_conditional(
        pool,
        None,
        Some(new_links),
        user.id,
    )
    .await?;
    assert_eq!(updated.name, "Conditional Test"); // name unchanged
    let links = updated.social_links.expect("Expected Some");
    assert_eq!(links[0].name, "Twitter");
    println!("✓ Conditional update changed only social_links (name skipped)");

    // 2. Update only name (social_links = None → skip)
    let updated2 = generated::users_array_fields::update_user_social_links_conditional(
        pool,
        Some("Conditional Updated".to_string()),
        None,
        user.id,
    )
    .await?;
    assert_eq!(updated2.name, "Conditional Updated");
    let links2 = updated2.social_links.expect("Expected Some");
    assert_eq!(links2[0].name, "Twitter"); // social_links unchanged
    println!("✓ Conditional update changed only name (social_links skipped)");

    // 3. Update both
    let both_links = vec![
        UserSocialLink {
            name: "LinkedIn".to_string(),
            url: "https://linkedin.com/in/condtest".to_string(),
        },
        UserSocialLink {
            name: "Blog".to_string(),
            url: "https://condtest.blog".to_string(),
        },
    ];
    let updated3 = generated::users_array_fields::update_user_social_links_conditional(
        pool,
        Some("Both Updated".to_string()),
        Some(both_links),
        user.id,
    )
    .await?;
    assert_eq!(updated3.name, "Both Updated");
    let links3 = updated3.social_links.expect("Expected Some");
    assert_eq!(links3.len(), 2);
    println!("✓ Conditional update changed both name and social_links");

    println!("\n✓ Conditional with JSONB test completed!");
    Ok(())
}

/// Test: multiunzip batch insert with Option<Vec<CustomStruct>> JSONB field
async fn test_social_links_batch(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::UserSocialLink;

    println!("Testing multiunzip batch insert with optional JSONB custom type...");
    let timestamp = chrono::Utc::now().timestamp();

    let items = vec![
        generated::users_array_fields::InsertUsersBatchSocialLinksRecord {
            name: "Batch User 1".to_string(),
            email: format!("batch1.social.{}@example.com", timestamp),
            social_links: Some(vec![UserSocialLink {
                name: "GitHub".to_string(),
                url: "https://github.com/batch1".to_string(),
            }]),
        },
        generated::users_array_fields::InsertUsersBatchSocialLinksRecord {
            name: "Batch User 2".to_string(),
            email: format!("batch2.social.{}@example.com", timestamp),
            social_links: None, // NULL social_links
        },
        generated::users_array_fields::InsertUsersBatchSocialLinksRecord {
            name: "Batch User 3".to_string(),
            email: format!("batch3.social.{}@example.com", timestamp),
            social_links: Some(vec![
                UserSocialLink {
                    name: "Twitter".to_string(),
                    url: "https://twitter.com/batch3".to_string(),
                },
                UserSocialLink {
                    name: "Website".to_string(),
                    url: "https://batch3.dev".to_string(),
                },
            ]),
        },
    ];

    let results =
        generated::users_array_fields::insert_users_batch_social_links(pool, items).await?;
    assert_eq!(results.len(), 3);

    // User 1: has social links
    let links1 = results[0]
        .social_links
        .as_ref()
        .expect("Expected Some for user 1");
    assert_eq!(links1.len(), 1);
    assert_eq!(links1[0].name, "GitHub");
    println!("✓ User 1: {} link(s)", links1.len());

    // User 2: NULL social links
    assert!(
        results[1].social_links.is_none(),
        "Expected None for user 2"
    );
    println!("✓ User 2: NULL social_links");

    // User 3: has social links
    let links3 = results[2]
        .social_links
        .as_ref()
        .expect("Expected Some for user 3");
    assert_eq!(links3.len(), 2);
    assert_eq!(links3[0].name, "Twitter");
    println!("✓ User 3: {} link(s)", links3.len());

    println!("\n✓ Batch insert with optional JSONB test completed!");
    println!(
        "  - Vec<Record> with Option<Vec<CustomStruct>> correctly handles mixed NULL/Some values"
    );
    Ok(())
}

// ====================================================================
// jsonb[] column tests — Vec<Option<UserTag>> (array of nullable JSONB)
// ====================================================================

/// Test: basic set/get for jsonb[] column
async fn test_tags(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::UserTag;

    println!("Testing jsonb[] column (Vec<Option<UserTag>>)...");
    let timestamp = chrono::Utc::now().timestamp();

    // Create a user first
    let user = generated::users::insert_user(
        pool,
        "Tags Test".to_string(),
        format!("tags.test.{}@example.com", timestamp),
        25,
        models::UserProfile {
            bio: None,
            avatar_url: None,
            preferences: models::UserPreferences {
                theme: "dark".to_string(),
                language: "en".to_string(),
                notifications_enabled: true,
            },
            social_links: vec![],
        },
    )
    .await?;
    println!("✓ Created user ID: {}", user.id);

    // 1. Set tags with mixed Some/None elements
    let tags = vec![
        Some(UserTag {
            label: "lang".to_string(),
            value: "rust".to_string(),
        }),
        None, // null element in jsonb[]
        Some(UserTag {
            label: "role".to_string(),
            value: "dev".to_string(),
        }),
    ];
    let updated = generated::users_array_fields::update_user_tags(pool, tags, user.id).await?;
    let result_tags = updated.tags.expect("Expected Some tags");
    assert_eq!(result_tags.len(), 3);
    assert!(result_tags[0].is_some());
    assert!(result_tags[1].is_none());
    assert!(result_tags[2].is_some());
    println!("✓ Set tags: [Some, None, Some] — null elements preserved");

    // 2. Read back
    let read = generated::users_array_fields::get_user_tags(pool, user.id).await?;
    let read_tags = read.tags.expect("Expected Some tags");
    assert_eq!(read_tags.len(), 3);
    assert_eq!(read_tags[0].as_ref().unwrap().label, "lang");
    assert!(read_tags[1].is_none());
    assert_eq!(read_tags[2].as_ref().unwrap().label, "role");
    println!("✓ Read back: values and nulls correct");

    // 3. Set to empty array
    let updated2 = generated::users_array_fields::update_user_tags(pool, vec![], user.id).await?;
    let empty_tags = updated2
        .tags
        .expect("Expected Some (empty array, not NULL)");
    assert_eq!(empty_tags.len(), 0);
    println!("✓ Set to empty array: len=0");

    println!("\n✓ jsonb[] basic test completed!");
    Ok(())
}

/// Test: parameters_type with jsonb[] column
async fn test_tags_structured(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::UserTag;

    println!("Testing parameters_type with jsonb[] column...");
    let timestamp = chrono::Utc::now().timestamp();

    let params = generated::users_array_fields::InsertUserTagsStructuredParams {
        name: "Tags Struct Test".to_string(),
        email: format!("tags.struct.{}@example.com", timestamp),
        tags: vec![
            Some(UserTag {
                label: "team".to_string(),
                value: "backend".to_string(),
            }),
            None,
        ],
    };

    let result = generated::users_array_fields::insert_user_tags_structured(pool, &params).await?;
    println!("✓ Inserted user ID: {}", result.id);
    let tags = result.tags.expect("Expected Some tags");
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].as_ref().unwrap().label, "team");
    assert!(tags[1].is_none());
    println!("✓ Tags round-tripped through parameters_type: [Some, None]");

    println!("\n✓ Structured parameters with jsonb[] test completed!");
    Ok(())
}

/// Test: conditions_type (diff) with jsonb[] column
async fn test_tags_diff(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::UserTag;

    println!("Testing conditions_type (diff) with jsonb[] column...");
    let timestamp = chrono::Utc::now().timestamp();

    // Create user with initial tags
    let user = generated::users::insert_user(
        pool,
        "Tags Diff Test".to_string(),
        format!("tags.diff.{}@example.com", timestamp),
        30,
        models::UserProfile {
            bio: None,
            avatar_url: None,
            preferences: models::UserPreferences {
                theme: "dark".to_string(),
                language: "en".to_string(),
                notifications_enabled: true,
            },
            social_links: vec![],
        },
    )
    .await?;

    // Set initial tags
    let initial_tags = vec![Some(UserTag {
        label: "lang".to_string(),
        value: "rust".to_string(),
    })];
    generated::users_array_fields::update_user_tags(pool, initial_tags.clone(), user.id).await?;
    println!("✓ Created user ID: {} with initial tags", user.id);

    let old = generated::users_array_fields::UpdateUserTagsDiffParams {
        name: "Tags Diff Test".to_string(),
        tags: initial_tags,
    };

    // 1. Change only tags
    let new_tags = vec![
        Some(UserTag {
            label: "lang".to_string(),
            value: "go".to_string(),
        }),
        None,
        Some(UserTag {
            label: "os".to_string(),
            value: "linux".to_string(),
        }),
    ];
    let new = generated::users_array_fields::UpdateUserTagsDiffParams {
        name: "Tags Diff Test".to_string(),
        tags: new_tags.clone(),
    };
    let updated =
        generated::users_array_fields::update_user_tags_diff(pool, &old, &new, user.id).await?;
    let tags = updated.tags.expect("Expected Some");
    assert_eq!(tags.len(), 3);
    assert!(tags[1].is_none());
    println!("✓ Diff: changed tags only (name unchanged)");

    // 2. No changes
    let updated2 =
        generated::users_array_fields::update_user_tags_diff(pool, &new, &new, user.id).await?;
    assert_eq!(updated2.name, "Tags Diff Test");
    println!("✓ Diff: no changes, returned current data");

    println!("\n✓ Conditional diff with jsonb[] test completed!");
    Ok(())
}

/// Test: conditional (no struct) with jsonb[] column
async fn test_tags_conditional(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::UserTag;

    println!("Testing conditional (non-diff) with jsonb[] column...");
    let timestamp = chrono::Utc::now().timestamp();

    // Create user
    let user = generated::users::insert_user(
        pool,
        "Tags Cond Test".to_string(),
        format!("tags.cond.{}@example.com", timestamp),
        28,
        models::UserProfile {
            bio: None,
            avatar_url: None,
            preferences: models::UserPreferences {
                theme: "dark".to_string(),
                language: "en".to_string(),
                notifications_enabled: true,
            },
            social_links: vec![],
        },
    )
    .await?;

    // Set initial tags
    generated::users_array_fields::update_user_tags(
        pool,
        vec![Some(UserTag {
            label: "init".to_string(),
            value: "true".to_string(),
        })],
        user.id,
    )
    .await?;
    println!("✓ Created user ID: {} with initial tags", user.id);

    // 1. Update only tags (name = None → skip)
    let new_tags = vec![
        Some(UserTag {
            label: "updated".to_string(),
            value: "yes".to_string(),
        }),
        None,
    ];
    let updated = generated::users_array_fields::update_user_tags_conditional(
        pool,
        None,
        Some(new_tags),
        user.id,
    )
    .await?;
    assert_eq!(updated.name, "Tags Cond Test");
    let tags = updated.tags.expect("Expected Some");
    assert_eq!(tags.len(), 2);
    assert!(tags[1].is_none());
    println!("✓ Conditional: changed only tags, name skipped");

    // 2. Update only name (tags = None → skip)
    let updated2 = generated::users_array_fields::update_user_tags_conditional(
        pool,
        Some("Tags Cond Updated".to_string()),
        None,
        user.id,
    )
    .await?;
    assert_eq!(updated2.name, "Tags Cond Updated");
    let tags2 = updated2.tags.expect("Expected Some");
    assert_eq!(tags2.len(), 2); // unchanged
    println!("✓ Conditional: changed only name, tags skipped");

    // 3. Update both
    let both_tags = vec![Some(UserTag {
        label: "final".to_string(),
        value: "done".to_string(),
    })];
    let updated3 = generated::users_array_fields::update_user_tags_conditional(
        pool,
        Some("Tags Both Updated".to_string()),
        Some(both_tags),
        user.id,
    )
    .await?;
    assert_eq!(updated3.name, "Tags Both Updated");
    let tags3 = updated3.tags.expect("Expected Some");
    assert_eq!(tags3.len(), 1);
    println!("✓ Conditional: changed both name and tags");

    println!("\n✓ Conditional with jsonb[] test completed!");
    Ok(())
}

/// Test: multiunzip batch insert with jsonb[] column
async fn test_tags_batch(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::UserTag;

    println!("Testing multiunzip batch insert with jsonb[] column...");
    let timestamp = chrono::Utc::now().timestamp();

    let items = vec![
        generated::users_array_fields::InsertUsersBatchTagsRecord {
            name: "Batch Tag 1".to_string(),
            email: format!("batch.tag1.{}@example.com", timestamp),
            tags: vec![
                Some(UserTag {
                    label: "lang".to_string(),
                    value: "rust".to_string(),
                }),
                None,
            ],
        },
        generated::users_array_fields::InsertUsersBatchTagsRecord {
            name: "Batch Tag 2".to_string(),
            email: format!("batch.tag2.{}@example.com", timestamp),
            tags: vec![], // empty array (not NULL)
        },
        generated::users_array_fields::InsertUsersBatchTagsRecord {
            name: "Batch Tag 3".to_string(),
            email: format!("batch.tag3.{}@example.com", timestamp),
            tags: vec![
                Some(UserTag {
                    label: "os".to_string(),
                    value: "linux".to_string(),
                }),
                Some(UserTag {
                    label: "editor".to_string(),
                    value: "vim".to_string(),
                }),
            ],
        },
    ];

    let results = generated::users_array_fields::insert_users_batch_tags(pool, items).await?;
    assert_eq!(results.len(), 3);

    // User 1: [Some, None]
    let tags1 = results[0].tags.as_ref().expect("Expected Some for user 1");
    assert_eq!(tags1.len(), 2);
    assert!(tags1[0].is_some());
    assert!(tags1[1].is_none());
    println!("✓ User 1: [Some, None] — null element preserved in batch");

    // User 2: empty array
    let tags2 = results[1].tags.as_ref().expect("Expected Some for user 2");
    assert_eq!(tags2.len(), 0);
    println!("✓ User 2: empty array []");

    // User 3: [Some, Some]
    let tags3 = results[2].tags.as_ref().expect("Expected Some for user 3");
    assert_eq!(tags3.len(), 2);
    assert_eq!(tags3[0].as_ref().unwrap().label, "os");
    assert_eq!(tags3[1].as_ref().unwrap().label, "editor");
    println!("✓ User 3: [Some, Some]");

    println!("\n✓ Batch insert with jsonb[] test completed!");
    println!("  - UNNEST with jsonb[] column works via ARRAY(SELECT jsonb_array_elements(...))");
    println!("  - Null elements within jsonb[] arrays preserved across batch insert");
    Ok(())
}

// ── Required jsonb[] column (labels) tests ────────────────────────

/// Test: basic set/get for required jsonb[] column (NOT NULL, Vec<Option<UserTag>>)
async fn test_labels(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::UserTag;

    println!("Testing required jsonb[] column (Vec<Option<UserTag>>)...");
    let timestamp = chrono::Utc::now().timestamp();

    // Create a user first
    let user = generated::users::insert_user(
        pool,
        "Labels Test".to_string(),
        format!("labels.test.{}@example.com", timestamp),
        25,
        models::UserProfile {
            bio: None,
            avatar_url: None,
            preferences: models::UserPreferences {
                theme: "dark".to_string(),
                language: "en".to_string(),
                notifications_enabled: true,
            },
            social_links: vec![],
        },
    )
    .await?;
    println!("✓ Created user ID: {}", user.id);

    // 1. Read default (should be empty array, NOT null)
    let read = generated::users_array_fields::get_user_labels(pool, user.id).await?;
    assert_eq!(read.labels.len(), 0);
    println!("✓ Default labels: empty array (NOT NULL)");

    // 2. Set labels with mixed Some/None elements
    let labels = vec![
        Some(UserTag {
            label: "lang".to_string(),
            value: "rust".to_string(),
        }),
        None,
        Some(UserTag {
            label: "role".to_string(),
            value: "dev".to_string(),
        }),
    ];
    let updated =
        generated::users_array_fields::update_user_labels(pool, labels, user.id).await?;
    assert_eq!(updated.labels.len(), 3);
    assert!(updated.labels[0].is_some());
    assert!(updated.labels[1].is_none());
    assert!(updated.labels[2].is_some());
    println!("✓ Set labels: [Some, None, Some] — null elements preserved");

    // 3. Read back
    let read = generated::users_array_fields::get_user_labels(pool, user.id).await?;
    assert_eq!(read.labels.len(), 3);
    assert_eq!(read.labels[0].as_ref().unwrap().label, "lang");
    assert!(read.labels[1].is_none());
    assert_eq!(read.labels[2].as_ref().unwrap().label, "role");
    println!("✓ Read back: values and nulls correct");

    // 4. Set to empty array
    let updated2 =
        generated::users_array_fields::update_user_labels(pool, vec![], user.id).await?;
    assert_eq!(updated2.labels.len(), 0);
    println!("✓ Set to empty array: len=0");

    println!("\n✓ Required jsonb[] basic test completed!");
    Ok(())
}

/// Test: parameters_type with required jsonb[] column
async fn test_labels_structured(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::UserTag;

    println!("Testing parameters_type with required jsonb[] column...");
    let timestamp = chrono::Utc::now().timestamp();

    let params = generated::users_array_fields::InsertUserLabelsStructuredParams {
        name: "Labels Struct Test".to_string(),
        email: format!("labels.struct.{}@example.com", timestamp),
        labels: vec![
            Some(UserTag {
                label: "team".to_string(),
                value: "backend".to_string(),
            }),
            None,
        ],
    };

    let result =
        generated::users_array_fields::insert_user_labels_structured(pool, &params).await?;
    println!("✓ Inserted user ID: {}", result.id);
    assert_eq!(result.labels.len(), 2);
    assert_eq!(result.labels[0].as_ref().unwrap().label, "team");
    assert!(result.labels[1].is_none());
    println!("✓ Labels round-tripped through parameters_type: [Some, None]");

    println!("\n✓ Structured parameters with required jsonb[] test completed!");
    Ok(())
}

/// Test: conditions_type (diff) with required jsonb[] column
async fn test_labels_diff(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::UserTag;

    println!("Testing conditions_type (diff) with required jsonb[] column...");
    let timestamp = chrono::Utc::now().timestamp();

    // Create user
    let user = generated::users::insert_user(
        pool,
        "Labels Diff Test".to_string(),
        format!("labels.diff.{}@example.com", timestamp),
        30,
        models::UserProfile {
            bio: None,
            avatar_url: None,
            preferences: models::UserPreferences {
                theme: "dark".to_string(),
                language: "en".to_string(),
                notifications_enabled: true,
            },
            social_links: vec![],
        },
    )
    .await?;

    // Set initial labels
    let initial_labels = vec![Some(UserTag {
        label: "lang".to_string(),
        value: "rust".to_string(),
    })];
    generated::users_array_fields::update_user_labels(pool, initial_labels.clone(), user.id)
        .await?;
    println!("✓ Created user ID: {} with initial labels", user.id);

    let old = generated::users_array_fields::UpdateUserLabelsDiffParams {
        name: "Labels Diff Test".to_string(),
        labels: initial_labels,
    };

    // 1. Change only labels
    let new_labels = vec![
        Some(UserTag {
            label: "lang".to_string(),
            value: "go".to_string(),
        }),
        None,
        Some(UserTag {
            label: "os".to_string(),
            value: "linux".to_string(),
        }),
    ];
    let new = generated::users_array_fields::UpdateUserLabelsDiffParams {
        name: "Labels Diff Test".to_string(),
        labels: new_labels.clone(),
    };
    let updated =
        generated::users_array_fields::update_user_labels_diff(pool, &old, &new, user.id).await?;
    assert_eq!(updated.labels.len(), 3);
    assert!(updated.labels[1].is_none());
    println!("✓ Diff: changed labels only (name unchanged)");

    // 2. No changes
    let updated2 =
        generated::users_array_fields::update_user_labels_diff(pool, &new, &new, user.id).await?;
    assert_eq!(updated2.name, "Labels Diff Test");
    println!("✓ Diff: no changes, returned current data");

    println!("\n✓ Conditional diff with required jsonb[] test completed!");
    Ok(())
}

/// Test: conditional (no struct) with required jsonb[] column
async fn test_labels_conditional(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::UserTag;

    println!("Testing conditional (non-diff) with required jsonb[] column...");
    let timestamp = chrono::Utc::now().timestamp();

    // Create user
    let user = generated::users::insert_user(
        pool,
        "Labels Cond Test".to_string(),
        format!("labels.cond.{}@example.com", timestamp),
        28,
        models::UserProfile {
            bio: None,
            avatar_url: None,
            preferences: models::UserPreferences {
                theme: "dark".to_string(),
                language: "en".to_string(),
                notifications_enabled: true,
            },
            social_links: vec![],
        },
    )
    .await?;

    // Set initial labels
    generated::users_array_fields::update_user_labels(
        pool,
        vec![Some(UserTag {
            label: "init".to_string(),
            value: "true".to_string(),
        })],
        user.id,
    )
    .await?;
    println!("✓ Created user ID: {} with initial labels", user.id);

    // 1. Update only labels (name = None → skip)
    let new_labels = vec![
        Some(UserTag {
            label: "updated".to_string(),
            value: "yes".to_string(),
        }),
        None,
    ];
    let updated = generated::users_array_fields::update_user_labels_conditional(
        pool,
        None,
        Some(new_labels),
        user.id,
    )
    .await?;
    assert_eq!(updated.name, "Labels Cond Test");
    assert_eq!(updated.labels.len(), 2);
    assert!(updated.labels[1].is_none());
    println!("✓ Conditional: changed only labels, name skipped");

    // 2. Update only name (labels = None → skip)
    let updated2 = generated::users_array_fields::update_user_labels_conditional(
        pool,
        Some("Labels Cond Updated".to_string()),
        None,
        user.id,
    )
    .await?;
    assert_eq!(updated2.name, "Labels Cond Updated");
    assert_eq!(updated2.labels.len(), 2); // unchanged
    println!("✓ Conditional: changed only name, labels skipped");

    // 3. Update both
    let both_labels = vec![Some(UserTag {
        label: "final".to_string(),
        value: "done".to_string(),
    })];
    let updated3 = generated::users_array_fields::update_user_labels_conditional(
        pool,
        Some("Labels Both Updated".to_string()),
        Some(both_labels),
        user.id,
    )
    .await?;
    assert_eq!(updated3.name, "Labels Both Updated");
    assert_eq!(updated3.labels.len(), 1);
    println!("✓ Conditional: changed both name and labels");

    println!("\n✓ Conditional with required jsonb[] test completed!");
    Ok(())
}

/// Test: multiunzip batch insert with required jsonb[] column
async fn test_labels_batch(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::models::UserTag;

    println!("Testing multiunzip batch insert with required jsonb[] column...");
    let timestamp = chrono::Utc::now().timestamp();

    let items = vec![
        generated::users_array_fields::InsertUsersBatchLabelsRecord {
            name: "Batch Label 1".to_string(),
            email: format!("batch.label1.{}@example.com", timestamp),
            labels: vec![
                Some(UserTag {
                    label: "lang".to_string(),
                    value: "rust".to_string(),
                }),
                None,
            ],
        },
        generated::users_array_fields::InsertUsersBatchLabelsRecord {
            name: "Batch Label 2".to_string(),
            email: format!("batch.label2.{}@example.com", timestamp),
            labels: vec![], // empty array
        },
        generated::users_array_fields::InsertUsersBatchLabelsRecord {
            name: "Batch Label 3".to_string(),
            email: format!("batch.label3.{}@example.com", timestamp),
            labels: vec![
                Some(UserTag {
                    label: "os".to_string(),
                    value: "linux".to_string(),
                }),
                Some(UserTag {
                    label: "editor".to_string(),
                    value: "vim".to_string(),
                }),
            ],
        },
    ];

    let results = generated::users_array_fields::insert_users_batch_labels(pool, items).await?;
    assert_eq!(results.len(), 3);

    // User 1: [Some, None]
    assert_eq!(results[0].labels.len(), 2);
    assert!(results[0].labels[0].is_some());
    assert!(results[0].labels[1].is_none());
    println!("✓ User 1: [Some, None] — null element preserved in batch");

    // User 2: empty array
    assert_eq!(results[1].labels.len(), 0);
    println!("✓ User 2: empty array []");

    // User 3: [Some, Some]
    assert_eq!(results[2].labels.len(), 2);
    assert_eq!(results[2].labels[0].as_ref().unwrap().label, "os");
    assert_eq!(results[2].labels[1].as_ref().unwrap().label, "editor");
    println!("✓ User 3: [Some, Some]");

    println!("\n✓ Batch insert with required jsonb[] test completed!");
    println!("  - Required jsonb[] column (NOT NULL) produces Vec directly, no Option wrapper");
    Ok(())
}
