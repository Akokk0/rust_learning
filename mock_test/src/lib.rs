// order.rs
pub struct Item {
    pub name: String,
    pub price: u32,
}

pub trait PaymentGateway {
    fn process_payment(&self, order: &Order, credit_card: &str) -> bool;
}

pub struct Order {
    pub items: Vec<Item>,
    pub payment_gateway: Box<dyn PaymentGateway>,
}

impl Order {
    pub fn total(&self) -> u32 {
        self.items.iter().map(|item| item.price).sum()
    }

    pub fn process_payment(&self, credit_card: &str) -> bool {
        self.payment_gateway.process_payment(self, credit_card)
    }
}

// test_order.rs
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;

    // 创建一个 PaymentGateway 的模拟对象
    mock! {
        pub PaymentGateway {}
        trait PaymentGateway {
            fn process_payment(&self, order: &Order, credit_card: &str) -> bool;
        }
    }

    #[test]
    fn test_process_payment() {
        let items = vec![
            Item {
                name: "item1".to_string(),
                price: 10,
            },
            Item {
                name: "item2".to_string(),
                price: 20,
            },
        ];

        let mut mock_payment_gateway = MockPaymentGateway::new();
        mock_payment_gateway
            .expect_process_payment()
            .withf(|order, credit_card| order.total() == 30 && credit_card == "1234567812345678")
            .times(1)
            .return_const(true);

        let order = Order {
            items,
            payment_gateway: Box::new(mock_payment_gateway),
        };
        let credit_card = "1234567812345678";

        // 调用 process_payment 方法
        let result = order.process_payment(credit_card);

        // 验证 PaymentGateway 的 process_payment 方法是否被正确调用
        assert!(result);
    }
}