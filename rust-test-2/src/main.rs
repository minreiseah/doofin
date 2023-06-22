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

// uncomment next line for backtrace
//use std::env;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    // uncomment next line for backtrace
    //env::set_var("RUST_BACKTRACE", "1");
    
    println!("Hello, world!");

    /* Default credentials are
    username: guest
    password: guest
    The username "goofy" with password "goofu" is something I set up on my local server */
    let connection_details = OpenConnectionArguments
        ::new("localhost", 5672, "goofy", "goofu");

    let connection = Connection::open(&connection_details).await.unwrap();
    connection.register_callback(DefaultConnectionCallback).await.unwrap(); // register_callback is allegedly important, but idk what it does - JY

    // declare channel
    // ::new is for transient channels (deleted when server restarts)
    // ::durable_client_named is for durable channels (stays when server restarts)

    let channel = connection.open_channel(None).await.unwrap();
    channel.register_callback(DefaultChannelCallback).await.unwrap(); // once again, idk what register_callback does

    // declare publisher channels
    let data_publisher = channel
        .exchange_declare(ExchangeDeclareArguments::new("data_pub", "fanout"))
        .await.unwrap();

    let strategy_publisher = channel
        .exchange_declare(ExchangeDeclareArguments::new("strategy_pub", "fanout"))
        .await.unwrap();

    let execution_publisher = channel
        .exchange_declare(ExchangeDeclareArguments::new("execution_pub", "fanout"))
        .await.unwrap();

    let portfolio_publisher = channel
        .exchange_declare(ExchangeDeclareArguments::new("portfolio_pub", "fanout"))
        .await.unwrap();
    // end declare publisher channels

    // declare message queues
    // queue_declare returns a (String, AmqpMessageCount, u32)
    let (strategy_sub_name, _, _) = channel
        .queue_declare(QueueDeclareArguments::new("strategy_sub"))
        .await.unwrap().unwrap();

    let (execution_sub_name, _, _) = channel
        .queue_declare(QueueDeclareArguments::new("execution_sub"))
        .await.unwrap().unwrap();

    let (portfolio_sub_name, _, _) = channel
        .queue_declare(QueueDeclareArguments::new("portfolio_sub"))
        .await.unwrap().unwrap();
    // end declare message queues

    // Bind queues to exchange
    channel.queue_bind(QueueBindArguments::new(
        &strategy_sub_name, "data_pub", "data_data")).await.unwrap();

    channel.queue_bind(QueueBindArguments::new(
        &strategy_sub_name, "portfolio_pub", "portfolio_data")).await.unwrap();

    channel.queue_bind(QueueBindArguments::new(
        &execution_sub_name, "strategy_pub", "strategy_data")).await.unwrap();

    channel.queue_bind(QueueBindArguments::new(
        &portfolio_sub_name, "execution_pub", "execution_data")).await.unwrap();
    // end bind queues to exchange

    println!("Publisher and subscriber queues and message channels initialised!");


    // End everything nicely.
    time::sleep(time::Duration::from_secs(3)).await;

    channel.close().await.unwrap();
    println!("Channel closed.");
    time::sleep(time::Duration::from_secs(1)).await;

    connection.close().await.unwrap();
    println!("Connection closed.");

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