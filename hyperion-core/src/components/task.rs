use crate::components::{
    comm::{EventReceiver, EventSender},
    store_operator::StoreOperator,
};

pub struct Task<ES, ER, O> {
    sender: ES,
    receiver: ER,
    operator: O,
}

impl<ES, ER, O: StoreOperator> Task<ES, ER, O>
where
    ES: EventSender,
    ER: EventReceiver,
    O: StoreOperator,
{
    pub fn task_loop(&mut self) {
        loop {
            self.receiver.receive();

            self.operator.operate();

            //self.sender.send(m);
        }
    }
}
