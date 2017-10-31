extern crate proguard;

use proguard::MappingView;


static MAPPING: &'static [u8] = include_bytes!("res/mapping.txt");
static MAPPING_WIN: &'static [u8] = include_bytes!("res/mappingwindows.txt");


#[test]
fn test_basic() {
    let mapping = MappingView::from_slice(MAPPING).unwrap();
    let cls = mapping.find_class("android.support.constraint.ConstraintLayout$a").unwrap();

    assert_eq!(cls.class_name(), "android.support.constraint.ConstraintLayout$LayoutParams");
    assert_eq!(cls.alias(), "android.support.constraint.ConstraintLayout$a");

    assert_eq!(&cls.get_field("b").unwrap().to_string(), "int guideEnd");
}

#[test]
fn test_basic_win() {
    let mapping = MappingView::from_slice(MAPPING_WIN).unwrap();
    let cls = mapping.find_class("android.support.constraint.ConstraintLayout$a").unwrap();

    assert_eq!(cls.class_name(), "android.support.constraint.ConstraintLayout$LayoutParams");
    assert_eq!(cls.alias(), "android.support.constraint.ConstraintLayout$a");

    assert_eq!(&cls.get_field("b").unwrap().to_string(), "int guideEnd");
}

#[test]
fn test_methods() {
    let mapping = MappingView::from_slice(MAPPING).unwrap();
    let cls = mapping.find_class("android.support.constraint.ConstraintLayout$a").unwrap();

    let methods = cls.get_methods("a", Some(1848));
    assert_eq!(methods.len(), 1);
    assert_eq!(methods[0].to_string(), "void validate()".to_string());
}

#[test]
fn test_methods_win() {
    let mapping = MappingView::from_slice(MAPPING_WIN).unwrap();
    let cls = mapping.find_class("android.support.constraint.ConstraintLayout$a").unwrap();

    let methods = cls.get_methods("a", Some(1848));
    assert_eq!(methods.len(), 1);
    assert_eq!(methods[0].to_string(), "void validate()".to_string());
}

#[test]
fn test_extra_methods() {
    let mapping = MappingView::from_slice(MAPPING).unwrap();
    let cls = mapping.find_class("android.support.constraint.a.e").unwrap();
    let methods = cls.get_methods("a", Some(261));
    assert_eq!(methods.len(), 1);
    assert_eq!(&methods[0].to_string(),
               "android.support.constraint.solver.ArrayRow getRow(int)");
}

#[test]
fn test_extra_methods_win() {
    let mapping = MappingView::from_slice(MAPPING_WIN).unwrap();
    let cls = mapping.find_class("android.support.constraint.a.e").unwrap();
    let methods = cls.get_methods("a", Some(261));
    assert_eq!(methods.len(), 1);
    assert_eq!(&methods[0].to_string(),
               "android.support.constraint.solver.ArrayRow getRow(int)");
}

#[test]
fn test_mapping_info() {
    let mapping = MappingView::from_slice(MAPPING).unwrap();
    assert_eq!(mapping.has_line_info(), true);
}

#[test]
fn test_mapping_info_win() {
    let mapping = MappingView::from_slice(MAPPING_WIN).unwrap();
    assert_eq!(mapping.has_line_info(), true);
}

#[test]
fn test_method_matches() {
    let mapping = MappingView::from_slice(MAPPING).unwrap();
    let cls = mapping.find_class("android.support.constraint.a.a").unwrap();
    let meths = cls.get_methods("a", Some(320));
    assert_eq!(meths.len(), 1);
    assert_eq!(meths[0].name(), "remove");

    let meths = cls.get_methods("a", Some(200));
    assert_eq!(meths.len(), 1);
    assert_eq!(meths[0].name(), "put");
}

#[test]
fn test_method_matches_win() {
    let mapping = MappingView::from_slice(MAPPING_WIN).unwrap();
    let cls = mapping.find_class("android.support.constraint.a.a").unwrap();
    let meths = cls.get_methods("a", Some(320));
    assert_eq!(meths.len(), 1);
    assert_eq!(meths[0].name(), "remove");

    let meths = cls.get_methods("a", Some(200));
    assert_eq!(meths.len(), 1);
    assert_eq!(meths[0].name(), "put");
}

#[test]
fn test_uuid() {
    let mapping = MappingView::from_slice(MAPPING).unwrap();
    assert_eq!(mapping.uuid(), "5cd8e873-1127-5276-81b7-8ff25043ecfd".parse().unwrap());
}

#[test]
fn test_uuid_win() {
    let mapping = MappingView::from_slice(MAPPING_WIN).unwrap();
    assert_eq!(mapping.uuid(), "71d468f2-0dc4-5017-9f12-1a81081913ef".parse().unwrap());
}
