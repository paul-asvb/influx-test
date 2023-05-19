use influx_client;

#[tokio::main]
async fn main() {
    let mut config = influx_client::apis::configuration::Configuration::new();

    config.api_key = Some(influx_client::apis::configuration::ApiKey {
        prefix: None,
        key: "9_f8S2t1Yr_CMtPy-zrFChx1_ZXGY-lcejos4JvqhicOBYq4OU8PUKAHlHWSD3nZWShGf4H8A84A3pPS06vZXQ==".to_owned(),
    });

    config.base_path = "http://localhost:8086".to_string();

    let ping = influx_client::apis::ping_api::get_ping(&config).await;

    dbg!(ping);

   // influx_client::apis::query_api::post_query(configuration, zap_trace_span, accept_encoding, content_type, org, org_id, query)

   //influx_client::apis::buckets_api::get_buckets(configuration, zap_trace_span, offset, limit, after, org, org_id, name, id);
}
