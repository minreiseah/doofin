use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicConsumeArguments, BasicPublishArguments, QueueBindArguments, QueueDeclareArguments,
        ExchangeDeclareArguments, BasicGetArguments
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
    let _data_publisher = channel
        .exchange_declare(ExchangeDeclareArguments::new("data_pub", "fanout"))
        .await.unwrap();

    let _strategy_publisher = channel
        .exchange_declare(ExchangeDeclareArguments::new("strategy_pub", "fanout"))
        .await.unwrap();

    let _execution_publisher = channel
        .exchange_declare(ExchangeDeclareArguments::new("execution_pub", "fanout"))
        .await.unwrap();

    let _portfolio_publisher = channel
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
    let data_data = "data_pub";
    channel.queue_bind(QueueBindArguments::new(
        &strategy_sub_name, data_data, "data_data")).await.unwrap();

    channel.queue_bind(QueueBindArguments::new(
        &strategy_sub_name, "portfolio_pub", "portfolio_data")).await.unwrap();

    channel.queue_bind(QueueBindArguments::new(
        &execution_sub_name, "strategy_pub", "strategy_data")).await.unwrap();

    channel.queue_bind(QueueBindArguments::new(
        &portfolio_sub_name, "execution_pub", "execution_data")).await.unwrap();
    // end bind queues to exchange

    println!("Pub/Sub queues and message channels initialised!");

    // Test messages
    let test_data_pub = String::from(
        r#"
            {
                "publisher": "data"
                "data": "Test data_pub"
            }
        "#,
    ).into_bytes();

    let _test_strategy_pub = String::from(
        r#"
            {
                "publisher": "strategy"
                "data": "Test strategy_pub"
            }
        "#,
    ).into_bytes();

    let _test_execution_pub = String::from(
        r#"
            {
                "publisher": "execution"
                "data": "Test execution_pub"
            }
        "#,
    ).into_bytes();

    let _test_portfolio_pub = String::from(
        r#"
            {
                "publisher": "data"
                "data": "Test portfolio_pub"
            }
        "#,
    ).into_bytes();
    // end test messages

    //let strategy_eat = BasicConsumeArguments::new("strategy_sub", "cons-tag");
    //channel.basic_consume(DefaultConsumer::new(strategy_eat.no_ack), strategy_eat).await.unwrap();

    // Publishes a message
    // Note: NEED TO CLEAN THIS UP!
    let strategy_eat = BasicPublishArguments::new("data_pub", data_data);
    channel.basic_publish(BasicProperties::default(), test_data_pub, strategy_eat)
        .await.unwrap();


    let noods = BasicConsumeArguments::new(&strategy_sub_name, "");
    //let temp = channel.basic_consume(DefaultConsumer::new(noods.no_ack), noods).await.unwrap();
    let temp = channel.basic_get(BasicGetArguments::new(&strategy_sub_name))
        .await.unwrap().unwrap().2;

    let messagered = String::from_utf8_lossy(&temp).to_string();

    println!("{}", messagered);

    // End everything nicely.
    time::sleep(time::Duration::from_secs(1)).await;

    channel.close().await.unwrap();
    println!("Channel closed.");
    time::sleep(time::Duration::from_secs(1)).await;

    connection.close().await.unwrap();
    println!("Connection closed.");

}