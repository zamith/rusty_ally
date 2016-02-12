table! {
    tasks {
        id -> Integer,
        task -> VarChar,
        detail -> Nullable<VarChar>,
        why -> Nullable<VarChar>,
        day -> Text,
        ordr -> Nullable<Integer>,
        last_ordr -> Nullable<Integer>,
        status -> VarChar,
    }
}
