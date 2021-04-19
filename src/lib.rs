mod common;

use crate::common::*;

#[derive(Debug, Deserialize)]
struct Parent {
    a: (),
    b: u8,
    c: i8,
    d: [u8; 3],
    e: Child,
    // e: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct Child {
    a: (),
    b: u8,
    c: i8,
    d: [u8; 3],
    // e: Vec<u8>,
}

pub fn start() -> Result<()> {
    let mut siv = Cursive::new();
    let view = make_tree::<Parent>(&Samples::new()).unwrap();

    let dialog = Dialog::around(view);

    siv.add_layer(dialog);
    siv.add_global_callback('q', |siv| siv.quit());
    siv.run();

    Ok(())
}

pub fn make_tree<'de, T>(samples: &'de Samples) -> Result<Box<dyn View>>
where
    T: Deserialize<'de> + std::fmt::Debug,
{
    let mut tracer = Tracer::new(TracerConfig::default());
    let (format, _values) = tracer
        .trace_type::<T>(samples)
        .map_err(|err| format_err!("{:?}", err))?;

    let registry = tracer.registry().map_err(|err| format_err!("{:?}", err))?;

    dbg!(&registry);

    let view = build_item(&registry, &format)?;

    Ok(view)
}

fn build_container(registry: &Registry, format: &ContainerFormat) -> Result<Box<dyn View>> {
    let view = match format {
        ContainerFormat::UnitStruct => TextView::new("()").into_boxed_view(),
        ContainerFormat::NewTypeStruct(fields) => {
            todo!();
        }
        ContainerFormat::TupleStruct(fields) => {
            let mut list = ListView::new();

            fields
                .iter()
                .enumerate()
                .try_for_each(|(index, format)| -> Result<_> {
                    let child_view = build_item(registry, format)?;
                    list.add_child(&format!("{}", index), child_view);
                    Ok(())
                })?;

            list.into_boxed_view()
        }
        ContainerFormat::Struct(fields) => {
            let mut list = ListView::new();

            fields.iter().try_for_each(|field| -> Result<_> {
                let Named {
                    name,
                    value: format,
                } = field;

                let child_view = build_item(registry, format)?;
                list.add_child(name, child_view);
                Ok(())
            })?;

            list.into_boxed_view()
        }
        ContainerFormat::Enum(variants) => {
            todo!();
        }
    };

    Ok(view)
}

fn build_item(registry: &Registry, format: &Format) -> Result<Box<dyn View>> {
    let view = match *format {
        Format::Variable(ref format) => {
            todo!();
        }
        Format::TypeName(ref name) => {
            let format = registry
                .get(name)
                .ok_or_else(|| format_err!("invalid type '{}'", name))?;

            let cell = Arc::new(RefCell::new(None));

            let container_view = build_container(registry, format)?;
            let mut dialog = Dialog::around(container_view);

            {
                let cell = cell.clone();
                dialog.add_button("Finish", move |siv| {
                    let view = siv.pop_layer().unwrap();
                    *cell.try_borrow_mut().unwrap() = Some(view);
                });
            }

            *cell.try_borrow_mut().unwrap() = Some(dialog.into_boxed_view());

            let button = {
                let cell = cell.clone();
                Button::new(name, move |siv| {
                    let view = cell.try_borrow_mut().unwrap().take().unwrap();

                    siv.add_layer(view);
                })
            };

            button.into_boxed_view()
        }
        Format::Unit => TextView::new("()").into_boxed_view(),
        Format::Bool => Checkbox::new().into_boxed_view(),
        Format::I8 => {
            let edit = EditView::new().fixed_width(20);
            edit.into_boxed_view()
        }
        Format::I16 => {
            let edit = EditView::new().fixed_width(20);
            edit.into_boxed_view()
        }
        Format::I32 => {
            let edit = EditView::new().fixed_width(20);
            edit.into_boxed_view()
        }
        Format::I64 => {
            let edit = EditView::new().fixed_width(20);
            edit.into_boxed_view()
        }
        Format::I128 => {
            let edit = EditView::new().fixed_width(20);
            edit.into_boxed_view()
        }
        Format::U8 => {
            let edit = EditView::new().fixed_width(20);
            edit.into_boxed_view()
        }
        Format::U16 => {
            let edit = EditView::new().fixed_width(20);
            edit.into_boxed_view()
        }
        Format::U32 => {
            let edit = EditView::new().fixed_width(20);
            edit.into_boxed_view()
        }
        Format::U64 => {
            let edit = EditView::new().fixed_width(20);
            edit.into_boxed_view()
        }
        Format::U128 => {
            let edit = EditView::new().fixed_width(20);
            edit.into_boxed_view()
        }
        Format::F32 => {
            let edit = EditView::new().fixed_width(20);
            edit.into_boxed_view()
        }
        Format::F64 => {
            let edit = EditView::new().fixed_width(20);
            edit.into_boxed_view()
        }
        Format::Char => {
            let edit = EditView::new().fixed_width(20);
            edit.into_boxed_view()
        }
        Format::Str => {
            let edit = EditView::new().fixed_width(20);
            edit.into_boxed_view()
        }
        Format::Bytes => {
            todo!();
        }
        Format::Option(ref format) => {
            todo!();
        }
        Format::Seq(ref format) => {
            todo!();
        }
        Format::Map { ref key, ref value } => {
            todo!();
        }
        Format::Tuple(ref formats) => {
            todo!();
        }
        Format::TupleArray { ref content, size } => {
            let list = {
                let mut list = ListView::new();

                (0..size).try_for_each(|index| -> Result<_> {
                    let view = build_item(registry, content)?;
                    list.add_child(&format!("{}", index), view);
                    Ok(())
                })?;

                list
            };

            let cell = Arc::new(RefCell::new(None));

            let dialog = {
                let cell = cell.clone();
                let mut dialog = Dialog::around(list);
                dialog.add_button("Finish", move |siv| {
                    let view = siv.pop_layer().unwrap();
                    *cell.try_borrow_mut().unwrap() = Some(view);
                });
                dialog
            };

            let button = {
                let cell = cell.clone();
                Button::new("Edit", move |siv| {
                    let view = cell.try_borrow_mut().unwrap().take().unwrap();
                    siv.add_layer(view);
                })
            };

            *cell.try_borrow_mut().unwrap() = Some(dialog.into_boxed_view());
            button.into_boxed_view()
        }
    };

    Ok(view)
}

// pub enum Node {
//     Unit(Option<Box<dyn View>>),
//     NewType(Option<Box<dyn View>>),
//     Tuple(Vec<Option<Box<dyn View>>>),
//     Struct(HashMap<String, Option<Box<dyn View>>>),
// }
