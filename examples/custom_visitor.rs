#![allow(dead_code)]

use field_kinds::{FieldInfo, FieldKinds, FieldVisitor, VisitFields};

#[derive(FieldKinds)]
struct Product {
    id: u64,
    name: String,
    price: f64,
    in_stock: bool,
    description: Option<String>,
    tags: Vec<String>,
}

struct NumericFieldCollector {
    fields: Vec<&'static str>,
}

impl FieldVisitor for NumericFieldCollector {
    fn visit<F: FieldInfo>(&mut self) {
        if F::CATEGORY_NAME == "numeric" {
            self.fields.push(F::NAME);
        }
    }
}

struct TypeNamePrinter;

impl FieldVisitor for TypeNamePrinter {
    fn visit<F: FieldInfo>(&mut self) {
        println!(
            "Field '{}' has type: {}",
            F::NAME,
            std::any::type_name::<F::Value>()
        );
    }
}

struct CategoryCounter {
    counts: std::collections::HashMap<&'static str, usize>,
}

impl FieldVisitor for CategoryCounter {
    fn visit<F: FieldInfo>(&mut self) {
        *self.counts.entry(F::CATEGORY_NAME).or_insert(0) += 1;
    }
}

fn main() {
    println!("=== Custom Visitor: Collect Numeric Fields ===");
    let mut collector = NumericFieldCollector { fields: vec![] };
    Product::visit_fields(&mut collector);
    println!("Numeric fields: {:?}", collector.fields);

    println!("\n=== Custom Visitor: Print Type Names ===");
    let mut printer = TypeNamePrinter;
    Product::visit_fields(&mut printer);

    println!("\n=== Custom Visitor: Count by Category ===");
    let mut counter = CategoryCounter {
        counts: std::collections::HashMap::new(),
    };
    Product::visit_fields(&mut counter);
    for (category, count) in &counter.counts {
        println!("  {category}: {count} field(s)");
    }
}
