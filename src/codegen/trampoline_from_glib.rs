use analysis::rust_type::rust_type;
use analysis::trampoline_parameters::Transformation;
use env::Env;
use library;
use traits::*;

pub trait TrampolineFromGlib {
    fn trampoline_from_glib(&self, env: &Env, need_downcast: bool) -> String;
}

impl TrampolineFromGlib for Transformation {
    fn trampoline_from_glib(&self, env: &Env, need_downcast: bool) -> String {
        use analysis::conversion_type::ConversionType::*;
        let need_type_name = need_downcast || is_need_type_name(env, self.typ);
        match self.conversion_type {
            Direct => self.name.clone(),
            Scalar => format!("from_glib({})", self.name),
            Borrow | Pointer => {
                let is_borrow = self.conversion_type == Borrow;
                let (mut left, mut right) = from_glib_xxx(self.transfer, is_borrow);
                if need_type_name {
                    let type_name = rust_type(env, self.typ).into_string();
                    left = format!("&{}::{}", type_name, left);
                } else {
                    left = format!("&{}", left);
                }
                if need_downcast {
                    right = format!("{}.downcast_unchecked()", right);
                }
                format!("{}{}{}", left, self.name, right)
            }
            Unknown => format!("/*Unknown conversion*/{}", self.name),
        }
    }
}

fn from_glib_xxx(transfer: library::Transfer, is_borrow: bool) -> (String, String) {
    use library::Transfer::*;
    match transfer {
        None if is_borrow => ("from_glib_borrow(".into(), ")".into()),
        None => ("from_glib_none(".into(), ")".into()),
        Full => ("from_glib_full(".into(), ")".into()),
        Container => ("from_glib_container(".into(), ")".into()),
    }
}

fn is_need_type_name(env: &Env, type_id: library::TypeId) -> bool {
    if type_id.ns_id == library::INTERNAL_NAMESPACE {
        use library::Type::*;
        use library::Fundamental::*;
        match *env.type_(type_id) {
            Fundamental(fund) if fund == Utf8 => true,
            Fundamental(fund) if fund == Filename => true,
            _ => false,
        }
    } else {
        false
    }
}
