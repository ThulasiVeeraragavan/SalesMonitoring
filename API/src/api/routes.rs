use actix_web::web;
use crate::api::endpoints::*;



pub fn init_routes_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(total_revenue);
    cfg.service(total_revenue_by_product);
    cfg.service(total_revenue_by_category);
    cfg.service(total_revenue_by_region);
    cfg.service(total_customers);
    cfg.service(total_orders);
    cfg.service(average_order_value);
    cfg.service(customer_lifetime_value);
    cfg.service(top_n_products_overall);
    cfg.service(top_n_products_by_category);
    cfg.service(top_n_products_by_region);
  }