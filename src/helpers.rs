use crate::schemas::{DependencyGroup, Package};

pub fn extract_packages_from_dependency_group(group: DependencyGroup) -> Vec<Package> {

    group.dependencies.unwrap_or_default().into_iter().map(|d|  
        Package {
        package_id: d.id,
        version: extract_verison_from_range(&d.range),
    }).collect()
}


pub fn extract_verison_from_range(range: &str) -> String {
    range.chars().filter(|c| !matches!(c, '[' | ',' | ' ' | ')')).collect()
}

#[test]
fn test_extract_version_from_range(){
    let range = "[8.0.0, )";
    let version = extract_verison_from_range(range);

    assert_eq!("8.0.0", version);
}