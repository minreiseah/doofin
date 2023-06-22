use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicConsumeArguments, BasicPublishArguments, QueueBindArguments, QueueDeclareArguments,
        ExchangeDeclareArguments
    },
    connection::{Connection, OpenConnectionArguments},
    consumer::DefaultConsumer,
    BasicProperties,
};
use tokio::time;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

// uncomment this for backtrace
//use std::env;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    
    println!("Hello, world!");

    // OpenConnectionArguments::new is the only important method for this struct
    let connectionDetails = OpenConnectionArguments::new(
        "localhost",
        5672,
        "goofy",
        "goofu"
    );

    let connection = Connection::open(&connectionDetails).await.unwrap();

    println!("check");

    // setting up an exchange
    let newExchange = ExchangeDeclareArguments::new("goof", "Fanout");

}

/*
tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .try_init()
        .ok();

    let fargs = OpenConnectionArguments::new(
        "amqps://xqxkltrn:rjhkj5eTc_kwSNQW2SLi4cQQLqs62VrC@armadillo.rmq.cloudamqp.com/xqxkltrn", // url
        5672,
        "xqxkltrn", // username
        "rjhkj5eTc_kwSNQW2SLi4cQQLqs62VrC", // password
    );

    let connection = Connection::open(&fargs).await.unwrap();

    println!("check");

    connection.register_callback(DefaultConnectionCallback).await.unwrap();

    // open a channel on the connection
    let channel = connection.open_channel(None).await.unwrap();
    channel.register_callback(DefaultChannelCallback).await.unwrap();

    // declare channel
    let (queue_name, _, _) = channel
        .queue_declare(QueueDeclareArguments::durable_client_named("amqprs.examples.basic"))
        .await
        .unwrap()
        .unwrap();

    // bind the queue to exchange
    let routing_key = "amqprs.example";
    let exchange_name = "amq.topic";
    channel.queue_bind(QueueBindArguments::new(
        &queue_name,
        exchange_name,
        routing_key
    )).await.unwrap();

    let args = BasicConsumeArguments::new(&queue_name, "example_basic_pub_sub");
    channel.basic_consume(DefaultConsumer::new(args.no_ack), args).await.unwrap();

    // publish message
    let content = String::from(
        r#"
            {
                "publisher": "example"
                "data": "Hello, amqprs!"
            }
        "#,
    ).into_bytes();

    // create args for basic publish
    let args = BasicPublishArguments::new(exchange_name, routing_key);
    channel.basic_publish(BasicProperties::default(), content, args).await.unwrap();

    time::sleep(time::Duration::from_secs(1)).await;

    channel.close().await.unwrap();
    connection.close().await.unwrap();
 */