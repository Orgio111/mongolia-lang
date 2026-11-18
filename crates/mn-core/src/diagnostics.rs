//! Draft Mongolian catalog; translations require native-speaker review before stabilization.
use crate::frontend::Error;

pub fn render_detail(error: &Error, lang: &str) -> String {
    if lang != "mn" {
        return error.detail.clone();
    }
    let detail = error.detail.as_str();
    let exact = match detail {
        "value outside i64 conversion range" => {
            Some("Утга бүхэл64 төрөлд хөрвүүлэх хязгаараас хэтэрсэн")
        }
        "invalid typed HIR" => Some("Төрөлжсөн HIR-ийн бүтэц буруу байна"),
        "numeric operand required for conversion" => {
            Some("Хөрвүүлэхдээ бүхэл64 эсвэл бодит64 төрлийн утга хэрэглэнэ үү")
        }
        "expression exceeds 256 nodes" => {
            Some("Нэг өгүүлбэр дэх илэрхийллийн тоо 256-аас хэтэрлээ")
        }
        "expression nesting exceeds 128"
        | "expression depth exceeded"
        | "evaluation depth exceeded" => Some("Илэрхийллийн гүн зөвшөөрөгдсөн хязгаараас хэтэрлээ"),
        "token limit exceeded" => Some("Токены тоо зөвшөөрөгдсөн хязгаараас хэтэрлээ"),
        "expected digits after decimal point" => Some("Аравтын цэгийн дараа цифр шаардлагатай"),
        "tensor allocation exceeds element limit or overflows" => {
            Some("Тензорын элементийн тоо зөвшөөрөгдсөн хязгаараас хэтэрлээ")
        }
        "tensor dimensions must be positive" => Some("Тензорын хэмжээс эерэг байх ёстой"),
        "call depth exceeded" => Some("Функцийн дуудлагын гүн хэтэрлээ"),
        "execution step budget exceeded" => Some("Гүйцэтгэлийн алхмын хязгаар хэтэрлээ"),
        "output budget exceeded" => Some("Гаралтын хэмжээ хязгаараас хэтэрлээ"),
        "frame allocation failed" => {
            Some("Функцийн дотоод хувьсагчдад санах ой хуваарилж чадсангүй")
        }
        "argument allocation failed" => Some("Аргументуудад санах ой хуваарилж чадсангүй"),
        "tensor allocation failed" => Some("Тензорт санах ой хуваарилж чадсангүй"),
        "output allocation failed" => Some("Гаралтад санах ой хуваарилж чадсангүй"),
        "string allocation failed" => Some("Мөрөнд санах ой хуваарилж чадсангүй"),
        "ragged tensor literal" => Some("Тензорын мөрүүдийн урт ижил байх ёстой"),
        "missing main function" => Some("main функц олдсонгүй"),
        "main must have no parameters and return unit" => {
            Some("main функц параметргүй, () буцах төрөлтэй байх ёстой")
        }
        "statement after return" => Some("Буцаах үйлдлийн дараах өгүүлбэр гүйцэтгэгдэхгүй"),
        "non-unit function must return a value" => Some("Энэ функц утга буцаах ёстой"),
        "integer division by zero" => Some("Бүхэл тоог тэгээр хувааж болохгүй"),
        "integer arithmetic overflow" | "integer negation overflow" => {
            Some("Бүхэл тооны үйлдлийн үр дүн зөвшөөрөгдсөн хязгаараас хэтэрлээ")
        }
        "Latin and Cyrillic characters cannot mix within one identifier" => {
            Some("Нэг таних тэмдэгтэд латин болон кирилл үсэг хольж болохгүй")
        }
        "unterminated block comment" => Some("Блок тайлбарын төгсгөл олдсонгүй"),
        "unterminated string" => Some("Мөрийн төгсгөлийн хашилт олдсонгүй"),
        "unterminated escape" => Some("Мөрийн тусгай тэмдэгтийн дараалал дутуу байна"),
        "unsupported escape" => Some("Дэмжигдээгүй тусгай тэмдэгтийн дараалал"),
        "newline in string" => Some("Мөрийн утгад шууд шинэ мөр оруулахыг зөвшөөрөхгүй"),
        "expected identifier" => Some("Таних тэмдэгт шаардлагатай"),
        "expected expression" => Some("Илэрхийлэл шаардлагатай"),
        "expected type" => Some("Төрөл шаардлагатай"),
        "unsupported type" => Some("Энэ хувилбарт дэмжигдээгүй төрөл"),
        "missing closing brace" => Some("Хаах } тэмдэг дутуу байна"),
        "expected fn or функц" => Some("fn эсвэл функц түлхүүр үг шаардлагатай"),
        "v0.1 tensors require f64 elements" => {
            Some("v0.1 тензорын элемент бодит64 төрөлтэй байх ёстой")
        }
        "dimension must be an integer in 1..=1000000" => {
            Some("Хэмжээс 1–1000000 завсрын бүхэл тоо байх ёстой")
        }
        "tensor size overflow" => Some("Тензорын хэмжээ тоон хязгаараас хэтэрлээ"),
        "@ requires two rank-2 tensors" => Some("@ үйлдэл хоёр хэмжээст хоёр тензор шаарддаг"),
        "arithmetic requires numeric operands" => {
            Some("Арифметик үйлдлийн операндууд тоон төрөлтэй байх ёстой")
        }
        "negation requires a scalar number" => Some("Тэмдэг солих үйлдэл скаляр тоо шаарддаг"),
        "integer literal out of range" => {
            Some("Бүхэл тоон бичлэг зөвшөөрөгдсөн хязгаараас хэтэрлээ")
        }
        "float literal out of range" => Some("Бодит тоон бичлэг зөвшөөрөгдсөн хязгаараас хэтэрлээ"),
        _ => None,
    };
    if let Some(text) = exact {
        return text.to_owned();
    }
    if let Some(names) = detail.strip_prefix("confusable identifiers ") {
        if let Some((first, second)) = names.split_once(" and ") {
            return format!("Харагдах байдлаараа төстэй нэрс: {first}, {second}");
        }
    }
    if let Some((shapes, reason)) = detail.split_once(": inner dimensions ") {
        if let Some(dimensions) = reason.strip_suffix(" are incompatible") {
            return format!(
                "{shapes}: дотоод хэмжээсүүд {} хоорондоо тохирохгүй",
                dimensions.replace(" and ", " ба ")
            );
        }
    }
    for (en, mn) in [
        ("undefined identifier ", "Тодорхойлогдоогүй таних тэмдэгт: "),
        ("undefined function ", "Тодорхойлогдоогүй функц: "),
        ("duplicate function ", "Давхардсан функц: "),
        ("duplicate parameter ", "Давхардсан параметр: "),
        ("duplicate binding ", "Давхардсан хувьсагч: "),
        ("expected '", "Шаардлагатай тэмдэг: '"),
        ("unexpected character ", "Зөвшөөрөгдөөгүй тэмдэг: "),
    ] {
        if let Some(rest) = detail.strip_prefix(en) {
            return format!("{mn}{rest}");
        }
    }
    if let Some(rest) = detail.strip_prefix("expected ") {
        if let Some((expected, actual)) = rest.split_once(", found ") {
            return format!("Шаардлагатай төрөл: {expected}; олдсон төрөл: {actual}");
        }
    }
    if let Some((name, rest)) = detail.split_once(" expects ") {
        if let Some((expected, actual)) = rest.split_once(" arguments, found ") {
            return format!(
                "{name}: {expected} аргумент шаардлагатай; өгсөн аргументын тоо {actual}"
            );
        }
    }
    // Preserve technical context for catalog entries that have not yet been translated.
    let category = match error.code {
        "E_LIMIT" => "Нөөцийн хязгаар хэтэрлээ",
        "E_ALLOCATION" => "Санах ой хуваарилах алдаа",
        "E_SHAPE" => "Тензорын хэмжээсийн алдаа",
        "E_TYPE" => "Төрлийн алдаа",
        "E_PARSE" => "Бичиглэлийн алдаа",
        "E_CONFUSABLE" => "Харагдах байдлаараа төстэй таних тэмдэгтүүд",
        "E_UTF8" => "UTF-8 кодчиллын алдаа",
        _ => "Програмын алдаа",
    };
    format!("{category}: {detail}")
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::Span;
    #[test]
    fn shape_catalog_keeps_evidence() {
        let error = Error {
            span: Span::default(),
            code: "E_SHAPE",
            detail: "[2, 3] @ [2, 2]: inner dimensions 3 and 2 are incompatible".into(),
        };
        assert_eq!(render_detail(&error, "en"), error.detail);
        assert_eq!(
            render_detail(&error, "mn"),
            "[2, 3] @ [2, 2]: дотоод хэмжээсүүд 3 ба 2 хоорондоо тохирохгүй"
        );
    }
}
