use repr_fits::repr_fits;

#[repr_fits(bits = 5)]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum RegionCode {
    Local = 0,
    Remote = 1,
    Backup = 4,
    Reserved = 7,
}

#[repr_fits(bits = 2)]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum PacketKind {
    Data = 0,
    Ack = 1,
}

#[test]
fn preserves_enum_definition() {
    assert_eq!(RegionCode::Backup as u8, 4);
    assert_eq!(PacketKind::Ack as u8, 1);
}

#[test]
fn keeps_variants_usable() {
    let code = RegionCode::Local;
    assert_eq!(code, RegionCode::Local);
    let _ = RegionCode::Remote;
    let _ = RegionCode::Reserved;
    let _ = PacketKind::Data;
}
