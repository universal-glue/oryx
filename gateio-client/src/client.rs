use crate::spot::Spot;

pub struct Client {

}

impl Client {
    pub fn spot(&self) -> Spot {
        Spot::with_transport()
    }

    /*

    fn margin(&self) -> Margin {

    }

    fn futures(&self) -> Futures {

    }

    fn delivery(&self) -> Delivery {

    }

     */
}