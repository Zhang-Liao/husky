[
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist::MnistLabel`, `Enum`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Type`),
        ),
    ),
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist::BinaryImage28`, `Struct`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Type`),
        ),
    ),
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist::input`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`{val_type} mnist::BinaryImage28`),
        ),
    ),
]