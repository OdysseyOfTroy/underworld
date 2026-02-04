use iced::widget::Column;

pub fn vert_stack<'a, Message>() -> Column<'a, Message> {
    Column::new().spacing(16)
}
