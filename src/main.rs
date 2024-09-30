#![windows_subsystem = "windows"]
use eframe::egui::CentralPanel;
use money::Money;
use rust_decimal::Decimal;
use unit::Unit;

mod money;
mod unit;
mod pronounce;

struct MyApp {
    budget: Money,
    price: Money,
    count: Unit
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp { budget: Money(Decimal::from(0)), price: Money(Decimal::from(0)), count: Unit(0) }
    }
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default()
            .show(ctx, |ui| {
                let summ = self.price * Money(self.count.0.into());
                let leftover = self.budget - self.price * self.count;

                ui.horizontal(
                    |ui| {
                        ui.label("Бюджет: ");
                        ui.text_edit_singleline(&mut self.budget);
                    });
                ui.label(format!("{}",self.budget));
                ui.horizontal(
                    |ui| {
                        ui.label("Цена: ");
                        ui.text_edit_singleline(&mut self.price);
                    });
                ui.label(format!("{}",self.price));
                ui.horizontal(
                    |ui| {
                        ui.label("Количество: ");
                        ui.text_edit_singleline(&mut self.count);
                    }
                );
                ui.label(format!("{}",self.count));
                ui.label(format!("Итоговая сумма: {} за {}.\n{} {}",
                    summ,
                    self.count,
                    if leftover.0.is_sign_positive() {
                        "В остатке:"
                    } else {
                        "Не хватает: "
                    },
                    leftover.abs()
                ));

            });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Money counter",
        options,
        Box::new(|ctx| {
            Ok( Box::new(MyApp::new(ctx) ))
        })           
    ).unwrap();
}


// fn main() -> anyhow::Result<()> {
//     let budget: Money = match args().nth(1) {
//         Some(val) => {
//             let dec = Decimal::from_str_exact(&val)?;
//             dec.try_into()?
//         },
//         None => {
//             Err(money::Error::BudgetNotPresented)?
//         },
//     };

//     const PRICE: Money = Money(Decimal::from_parts(8999,0 ,0 ,false , 2));

//     let count = budget.into() / PRICE.into();
//     let units = Unit(count.trunc().try_into()?);

//     let summ: Money = Money(
//         PRICE.into() * Decimal::from(&units)
//     );

//     let leftover = Money(
//         budget.0 - summ.0
//     );

//     println!("Размер бюджета: {}",budget);
//     println!("Цена за единицу товара: {}",PRICE);


//     print!("За {} можно приобрести {} товара ",budget,units);
//     println!("на сумму {}, имея в остатке {}",summ,leftover);


//     Ok(())
    
// }
