//! Real-World Complex Scenarios Testing
//!
//! This test file covers realistic, production-grade scenarios that developers
//! might encounter when using TypeStateBuilder in real applications.

use std::collections::{BTreeMap, HashMap};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use type_state_builder::TypeStateBuilder;

// ===== HTTP CLIENT CONFIGURATION =====

#[test]
fn test_http_client_configuration() {
    use std::time::Duration;

    trait HttpClient {
        type Request;
        type Response;
    }

    struct MockHttpClient;
    impl HttpClient for MockHttpClient {
        type Request = String;
        type Response = String;
    }

    #[derive(TypeStateBuilder)]
    #[builder(build_method = "create_client")]
    struct HttpClientConfig<C: HttpClient> {
        #[builder(required)]
        base_url: String,

        #[builder(required)]
        api_key: String,

        #[builder(skip_setter, default = "PhantomData")]
        client_type: PhantomData<C>,

        #[builder(default = "Duration::from_secs(30)")]
        timeout: Duration,

        #[builder(default = "10")]
        max_retries: u32,

        #[builder(default = "HashMap::new()")]
        default_headers: HashMap<String, String>,

        user_agent: Option<String>,
        proxy_url: Option<String>,

        #[builder(default = "true")]
        verify_ssl: bool,
    }

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let config = HttpClientConfig::<MockHttpClient>::builder()
        .base_url("https://api.example.com".to_string())
        .api_key("secret_key_123".to_string())
        .user_agent(Some("MyApp/1.0".to_string()))
        .proxy_url(Some("http://proxy.example.com:8080".to_string()))
        .default_headers(headers.clone())
        .create_client();

    assert_eq!(config.base_url, "https://api.example.com");
    assert_eq!(config.api_key, "secret_key_123");
    assert_eq!(config.timeout, Duration::from_secs(30));
    assert_eq!(config.max_retries, 10);
    assert!(config.verify_ssl);
    assert_eq!(config.user_agent, Some("MyApp/1.0".to_string()));
    assert_eq!(
        config.proxy_url,
        Some("http://proxy.example.com:8080".to_string())
    );
    assert_eq!(config.default_headers, headers);
    // Test client_type field - PhantomData doesn't have meaningful equality, but we can verify it's present
    assert_eq!(std::mem::size_of_val(&config.client_type), 0); // PhantomData is zero-sized
}

// ===== DATABASE CONNECTION POOL =====

#[test]
fn test_database_connection_pool() {
    trait DatabaseDriver {
        type Connection: Send + Sync;
        type Error: std::fmt::Debug + Send + Sync;
    }

    struct PostgresDriver;
    impl DatabaseDriver for PostgresDriver {
        type Connection = String; // Mock connection
        type Error = String;
    }

    #[derive(TypeStateBuilder)]
    #[builder(build_method = "create_pool")]
    struct ConnectionPoolConfig<'a, D: DatabaseDriver> {
        #[builder(required)]
        database_url: &'a str,

        #[builder(required)]
        max_connections: usize,

        #[builder(skip_setter, default = "PhantomData")]
        driver: PhantomData<D>,

        #[builder(default = "1")]
        min_connections: usize,

        #[builder(default = "std::time::Duration::from_secs(30)")]
        connection_timeout: std::time::Duration,

        #[builder(default = "std::time::Duration::from_secs(600)")]
        idle_timeout: std::time::Duration,

        #[builder(default = "true")]
        test_on_checkout: bool,

        initialization_query: Option<String>,

        #[builder(skip_setter, default = "Arc::new(Mutex::new(Vec::new()))")]
        connection_pool: Arc<Mutex<Vec<D::Connection>>>,
    }

    let config = ConnectionPoolConfig::<PostgresDriver>::builder()
        .database_url("postgresql://localhost:5432/mydb")
        .max_connections(20)
        .min_connections(5)
        .initialization_query(Some("SET search_path TO public".to_string()))
        .create_pool();

    assert_eq!(config.database_url, "postgresql://localhost:5432/mydb");
    assert_eq!(config.max_connections, 20);
    assert_eq!(config.min_connections, 5);
    assert!(config.test_on_checkout);
    assert_eq!(
        config.connection_timeout,
        std::time::Duration::from_secs(30)
    );
    assert_eq!(config.idle_timeout, std::time::Duration::from_secs(600));
    assert_eq!(
        config.initialization_query,
        Some("SET search_path TO public".to_string())
    );
    assert!(config.connection_pool.lock().unwrap().is_empty());
    // Test driver field - PhantomData doesn't have meaningful equality, but we can verify it's present
    assert_eq!(std::mem::size_of_val(&config.driver), 0); // PhantomData is zero-sized
}

// ===== ASYNC TASK SCHEDULER =====

#[test]
fn test_async_task_scheduler() {
    trait AsyncTask: Send + Sync {
        type Output: Send + Sync;
        type Future: Future<Output = Self::Output> + Send;

        fn execute(&self) -> Self::Future;
    }

    struct MockTask;
    impl AsyncTask for MockTask {
        type Output = String;
        type Future = Pin<Box<dyn Future<Output = String> + Send>>;

        fn execute(&self) -> Self::Future {
            Box::pin(async { "completed".to_string() })
        }
    }

    #[derive(TypeStateBuilder)]
    #[builder(build_method = "start_scheduler")]
    struct TaskScheduler<'a, T, F, E>
    where
        T: AsyncTask,
        F: Fn(T::Output) -> Result<(), E> + Send + Sync + 'a,
        E: std::fmt::Debug + Send + Sync,
    {
        #[builder(required)]
        task: T,

        #[builder(required)]
        callback: F,

        #[builder(required)]
        scheduler_name: String,

        #[builder(skip_setter, default = "PhantomData")]
        error_type: PhantomData<E>,

        #[builder(skip_setter, default = "PhantomData")]
        lifetime_marker: PhantomData<&'a ()>,

        #[builder(default = "1")]
        max_concurrent_tasks: usize,

        #[builder(default = "std::time::Duration::from_millis(100)")]
        poll_interval: std::time::Duration,

        retry_policy: Option<fn(u32) -> std::time::Duration>,

        #[builder(default = "None")]
        max_retries: Option<u32>,
    }

    let scheduler = TaskScheduler::<MockTask, _, String>::builder()
        .task(MockTask)
        .callback(|output: String| {
            if output == "completed" {
                Ok(())
            } else {
                Err("Failed".to_string())
            }
        })
        .scheduler_name("test_scheduler".to_string())
        .max_concurrent_tasks(4)
        .max_retries(Some(3))
        .start_scheduler();

    assert_eq!(scheduler.scheduler_name, "test_scheduler");
    assert_eq!(scheduler.max_concurrent_tasks, 4);
    assert_eq!(scheduler.max_retries, Some(3));

    // Test unused fields - poll_interval, retry_policy
    assert_eq!(
        scheduler.poll_interval,
        std::time::Duration::from_millis(100)
    );
    assert!(scheduler.retry_policy.is_none());

    // Test callback by calling it
    let callback_result = (scheduler.callback)("completed".to_string());
    assert!(callback_result.is_ok());

    // Test execute method on the task directly - create a simple runtime to avoid tokio dependency
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll, Waker};

    // Create a mock waker for testing
    fn dummy_waker() -> Waker {
        use std::task::{RawWaker, RawWakerVTable};

        fn no_op(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }

        const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
        let raw_waker = RawWaker::new(std::ptr::null(), &VTABLE);
        unsafe { Waker::from_raw(raw_waker) }
    }

    // Test the execute method
    let mut future = scheduler.task.execute();
    let waker = dummy_waker();
    let mut context = Context::from_waker(&waker);

    // Poll the future - it should be ready immediately in our mock
    match Pin::new(&mut future).poll(&mut context) {
        Poll::Ready(result) => {
            assert_eq!(result, "completed");
        }
        Poll::Pending => {
            // For testing purposes, we'll accept pending as it means the method was called
            // In a real application, we'd wait for completion
        }
    }

    // Test PhantomData fields - these are zero-sized but we verify they're present
    assert_eq!(std::mem::size_of_val(&scheduler.error_type), 0);
    assert_eq!(std::mem::size_of_val(&scheduler.lifetime_marker), 0);
}

// ===== CONFIGURATION MANAGEMENT SYSTEM =====

#[test]
fn test_configuration_management() {
    use std::path::PathBuf;

    trait ConfigSource {
        type Error: std::fmt::Debug;
        fn load(&self) -> Result<HashMap<String, String>, Self::Error>;
    }

    struct FileSource {
        path: PathBuf,
    }

    impl ConfigSource for FileSource {
        type Error = String;
        fn load(&self) -> Result<HashMap<String, String>, Self::Error> {
            Ok(HashMap::new()) // Mock implementation
        }
    }

    #[derive(TypeStateBuilder)]
    #[builder(build_method = "build_config_manager")]
    struct ConfigManager<S: ConfigSource, F, G>
    where
        F: Fn(&str) -> Option<String> + Send + Sync,
        G: Fn(&str, &str) + Send + Sync,
    {
        #[builder(required)]
        primary_source: S,

        #[builder(required)]
        value_transformer: F,

        #[builder(required)]
        change_listener: G,

        backup_sources: Option<Vec<S>>,

        #[builder(default = "BTreeMap::new()")]
        cached_values: BTreeMap<String, String>,

        #[builder(default = "std::time::Duration::from_secs(60)")]
        refresh_interval: std::time::Duration,

        #[builder(default = "true")]
        auto_refresh: bool,

        #[builder(skip_setter, default = "std::time::SystemTime::now()")]
        last_refresh: std::time::SystemTime,
    }

    let file_source = FileSource {
        path: PathBuf::from("/etc/myapp/config.toml"),
    };

    // Test unused field - path
    assert_eq!(file_source.path, PathBuf::from("/etc/myapp/config.toml"));

    let config_manager = ConfigManager::builder()
        .primary_source(file_source)
        .value_transformer(|key: &str| {
            if key == "database_url" {
                Some("transformed_url".to_string())
            } else {
                None
            }
        })
        .change_listener(|key: &str, value: &str| {
            println!("Config changed: {key} = {value}");
        })
        .auto_refresh(false)
        .build_config_manager();

    assert!(!config_manager.auto_refresh);
    assert!(config_manager.cached_values.is_empty());

    // Test unused trait method - load
    let load_result = config_manager.primary_source.load();
    assert!(load_result.is_ok());
    assert!(load_result.unwrap().is_empty());

    // Test unused fields - primary_source (already tested above), value_transformer, change_listener, backup_sources, refresh_interval, last_refresh
    let transformer_result = (config_manager.value_transformer)("database_url");
    assert_eq!(transformer_result, Some("transformed_url".to_string()));

    let transformer_result2 = (config_manager.value_transformer)("unknown_key");
    assert_eq!(transformer_result2, None);

    // Test change_listener by calling it
    (config_manager.change_listener)("test_key", "test_value");

    assert!(config_manager.backup_sources.is_none());
    assert_eq!(
        config_manager.refresh_interval,
        std::time::Duration::from_secs(60)
    );
    assert!(config_manager.last_refresh <= std::time::SystemTime::now());
}

// ===== EVENT-DRIVEN ARCHITECTURE =====

#[test]
fn test_event_driven_system() {
    trait Event: Send + Sync + std::fmt::Debug {
        fn event_type(&self) -> &'static str;
    }

    #[derive(Debug)]
    struct UserRegisteredEvent {
        user_id: u64,
        email: String,
    }

    impl Event for UserRegisteredEvent {
        fn event_type(&self) -> &'static str {
            "user_registered"
        }
    }

    trait EventHandler<E: Event> {
        type Error: std::fmt::Debug;
        fn handle(&self, event: E) -> Result<(), Self::Error>;
    }

    struct EmailNotificationHandler;
    impl EventHandler<UserRegisteredEvent> for EmailNotificationHandler {
        type Error = String;
        fn handle(&self, event: UserRegisteredEvent) -> Result<(), Self::Error> {
            println!("Sending welcome email to {}", event.email);
            Ok(())
        }
    }

    #[derive(TypeStateBuilder)]
    #[builder(build_method = "create_event_bus")]
    struct EventBus<'a, E, H, F>
    where
        E: Event,
        H: EventHandler<E>,
        F: Fn(&E, &H::Error) + Send + Sync + 'a,
    {
        #[builder(required)]
        event_type_name: String,

        #[builder(required)]
        handlers: Vec<H>,

        #[builder(required)]
        error_callback: F,

        #[builder(skip_setter, default = "PhantomData")]
        event_phantom: PhantomData<E>,

        #[builder(skip_setter, default = "PhantomData")]
        lifetime_phantom: PhantomData<&'a ()>,

        #[builder(default = "true")]
        async_processing: bool,

        #[builder(default = "1000")]
        max_queue_size: usize,

        #[builder(default = "std::time::Duration::from_millis(10)")]
        batch_timeout: std::time::Duration,

        dead_letter_queue: Option<String>,
    }

    let event_bus = EventBus::<UserRegisteredEvent, EmailNotificationHandler, _>::builder()
        .event_type_name("user_events".to_string())
        .handlers(vec![EmailNotificationHandler])
        .error_callback(|event: &UserRegisteredEvent, error: &String| {
            eprintln!("Failed to handle event {event:?}: {error}");
        })
        .max_queue_size(5000)
        .dead_letter_queue(Some("failed_events".to_string()))
        .create_event_bus();

    assert_eq!(event_bus.event_type_name, "user_events");
    assert_eq!(event_bus.max_queue_size, 5000);
    assert!(event_bus.async_processing);

    // Test unused trait methods - event_type and handle
    let test_event = UserRegisteredEvent {
        user_id: 123,
        email: "test@example.com".to_string(),
    };
    assert_eq!(test_event.event_type(), "user_registered");

    // Test unused fields - user_id and email
    assert_eq!(test_event.user_id, 123);
    assert_eq!(test_event.email, "test@example.com");

    let handler = EmailNotificationHandler;
    let handle_result = handler.handle(test_event);
    assert!(handle_result.is_ok());

    // Test unused fields - handlers, error_callback, batch_timeout, dead_letter_queue
    assert_eq!(event_bus.handlers.len(), 1);
    assert_eq!(
        event_bus.batch_timeout,
        std::time::Duration::from_millis(10)
    );
    assert_eq!(
        event_bus.dead_letter_queue,
        Some("failed_events".to_string())
    );

    // Test error_callback by calling it
    let test_event2 = UserRegisteredEvent {
        user_id: 456,
        email: "test2@example.com".to_string(),
    };
    let test_error = "Test error".to_string();
    (event_bus.error_callback)(&test_event2, &test_error);

    // Test PhantomData fields - these are zero-sized but we verify they're present
    assert_eq!(std::mem::size_of_val(&event_bus.event_phantom), 0);
    assert_eq!(std::mem::size_of_val(&event_bus.lifetime_phantom), 0);
}

// ===== MICROSERVICE COMMUNICATION =====

#[test]
fn test_microservice_communication() {
    trait Serializer {
        type Error: std::fmt::Debug;
        fn serialize<T: serde::Serialize>(&self, value: &T) -> Result<Vec<u8>, Self::Error>;
        fn deserialize<T: serde::de::DeserializeOwned>(
            &self,
            data: &[u8],
        ) -> Result<T, Self::Error>;
    }

    struct JsonSerializer;
    impl Serializer for JsonSerializer {
        type Error = String;
        fn serialize<T: serde::Serialize>(&self, _value: &T) -> Result<Vec<u8>, Self::Error> {
            Ok(b"{}".to_vec()) // Mock
        }
        fn deserialize<T: serde::de::DeserializeOwned>(
            &self,
            _data: &[u8],
        ) -> Result<T, Self::Error> {
            Err("Not implemented".to_string())
        }
    }

    trait Transport {
        type Error: std::fmt::Debug;
        fn send(&self, destination: &str, data: &[u8]) -> Result<(), Self::Error>;
        fn receive(&self) -> Result<Vec<u8>, Self::Error>;
    }

    struct HttpTransport;
    impl Transport for HttpTransport {
        type Error = String;
        fn send(&self, _destination: &str, _data: &[u8]) -> Result<(), Self::Error> {
            Ok(())
        }
        fn receive(&self) -> Result<Vec<u8>, Self::Error> {
            Ok(b"{}".to_vec())
        }
    }

    #[derive(TypeStateBuilder)]
    #[builder(build_method = "create_service_client")]
    struct ServiceClient<'a, S, T, F, G>
    where
        S: Serializer,
        T: Transport,
        F: Fn(&S::Error) + Send + Sync + 'a,
        G: Fn(&T::Error) + Send + Sync + 'a,
    {
        #[builder(required)]
        service_name: String,

        #[builder(required)]
        base_endpoint: String,

        #[builder(required)]
        serializer: S,

        #[builder(required)]
        transport: T,

        #[builder(required)]
        serialization_error_handler: F,

        #[builder(required)]
        transport_error_handler: G,

        #[builder(skip_setter, default = "PhantomData")]
        lifetime_phantom: PhantomData<&'a ()>,

        #[builder(default = "std::time::Duration::from_secs(30)")]
        request_timeout: std::time::Duration,

        #[builder(default = "3")]
        max_retries: u32,

        #[builder(default = "HashMap::new()")]
        service_registry: HashMap<String, String>,

        circuit_breaker_threshold: Option<u32>,

        #[builder(default = "\"1.0\".to_string()")]
        api_version: String,
    }

    let service_client = ServiceClient::builder()
        .service_name("user-service".to_string())
        .base_endpoint("http://user-service:8080".to_string())
        .serializer(JsonSerializer)
        .transport(HttpTransport)
        .serialization_error_handler(|error: &String| {
            eprintln!("Serialization error: {error}");
        })
        .transport_error_handler(|error: &String| {
            eprintln!("Transport error: {error}");
        })
        .circuit_breaker_threshold(Some(10))
        .api_version("2.0".to_string())
        .create_service_client();

    assert_eq!(service_client.service_name, "user-service");
    assert_eq!(service_client.api_version, "2.0");
    assert_eq!(service_client.circuit_breaker_threshold, Some(10));

    // Test unused trait methods - serialize, deserialize, send, receive
    let test_data = "test";
    let serialized = service_client.serializer.serialize(&test_data);
    assert!(serialized.is_ok());

    // Test deserialize method
    let deserialize_result: Result<String, _> = service_client.serializer.deserialize(b"{}");
    assert!(deserialize_result.is_err()); // Mock implementation returns error

    let send_result = service_client.transport.send("test-endpoint", &[1, 2, 3]);
    assert!(send_result.is_ok());

    let receive_result = service_client.transport.receive();
    assert!(receive_result.is_ok());

    // Test unused fields - base_endpoint, serializer (tested above), transport (tested above), serialization_error_handler, transport_error_handler, service_registry, request_timeout, max_retries
    assert_eq!(service_client.base_endpoint, "http://user-service:8080");
    assert!(service_client.service_registry.is_empty());
    assert_eq!(
        service_client.request_timeout,
        std::time::Duration::from_secs(30)
    );
    assert_eq!(service_client.max_retries, 3);

    // Test error handlers by calling them
    let test_error = "Test serialization error".to_string();
    (service_client.serialization_error_handler)(&test_error);

    let test_transport_error = "Test transport error".to_string();
    (service_client.transport_error_handler)(&test_transport_error);

    // Test PhantomData field - zero-sized but we verify it's present
    assert_eq!(std::mem::size_of_val(&service_client.lifetime_phantom), 0);
}

// ===== CACHING LAYER WITH TTL =====

#[test]
fn test_distributed_cache_system() {
    use std::time::Duration;

    trait CacheBackend {
        type Error: std::fmt::Debug;
        fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Self::Error>;
        fn set(&self, key: &str, value: &[u8], ttl: Duration) -> Result<(), Self::Error>;
        fn delete(&self, key: &str) -> Result<(), Self::Error>;
    }

    struct RedisBackend;
    impl CacheBackend for RedisBackend {
        type Error = String;
        fn get(&self, _key: &str) -> Result<Option<Vec<u8>>, Self::Error> {
            Ok(Some(b"cached_value".to_vec()))
        }
        fn set(&self, _key: &str, _value: &[u8], _ttl: Duration) -> Result<(), Self::Error> {
            Ok(())
        }
        fn delete(&self, _key: &str) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[derive(TypeStateBuilder)]
    #[builder(build_method = "create_cache")]
    struct DistributedCache<B: CacheBackend, H, S, F>
    where
        H: Fn(&str) -> String + Send + Sync, // Hash function
        S: Fn(&[u8]) -> Result<Vec<u8>, String> + Send + Sync, // Serializer
        F: Fn(&B::Error) + Send + Sync,      // Error handler
    {
        #[builder(required)]
        backends: Vec<B>,

        #[builder(required)]
        hash_function: H,

        #[builder(required)]
        serializer: S,

        #[builder(required)]
        error_handler: F,

        #[builder(default = "Duration::from_secs(3600)")]
        default_ttl: Duration,

        #[builder(default = "\"cache_v1\".to_string()")]
        key_prefix: String,

        #[builder(default = "true")]
        compression_enabled: bool,

        #[builder(default = "1000")]
        max_key_size: usize,

        #[builder(default = "10_000_000")]
        max_value_size: usize,

        #[builder(skip_setter, default = "HashMap::new()")]
        local_stats: HashMap<String, u64>,

        replication_factor: Option<usize>,
    }

    let cache = DistributedCache::builder()
        .backends(vec![RedisBackend, RedisBackend])
        .hash_function(|key: &str| format!("hash_{key}"))
        .serializer(|data: &[u8]| Ok(data.to_vec())) // Identity function
        .error_handler(|error: &String| {
            eprintln!("Cache error: {error}");
        })
        .key_prefix("myapp".to_string())
        .replication_factor(Some(2))
        .max_value_size(50_000_000)
        .create_cache();

    assert_eq!(cache.backends.len(), 2);
    assert_eq!(cache.key_prefix, "myapp");
    assert_eq!(cache.replication_factor, Some(2));
    assert!(cache.compression_enabled);

    // Test unused trait methods - get, set, delete
    let backend = &cache.backends[0];
    let get_result = backend.get("test_key");
    assert!(get_result.is_ok());

    let set_result = backend.set("test_key", b"test_value", Duration::from_secs(60));
    assert!(set_result.is_ok());

    let delete_result = backend.delete("test_key");
    assert!(delete_result.is_ok());

    // Test unused fields - hash_function, serializer, error_handler, local_stats, default_ttl, max_key_size, max_value_size
    let hash_result = (cache.hash_function)("test_key");
    assert_eq!(hash_result, "hash_test_key");

    let serialize_result = (cache.serializer)(b"test_data");
    assert!(serialize_result.is_ok());
    assert_eq!(serialize_result.unwrap(), b"test_data");

    // Test error_handler by calling it
    let test_error = "Test cache error".to_string();
    (cache.error_handler)(&test_error);

    assert!(cache.local_stats.is_empty());
    assert_eq!(cache.default_ttl, Duration::from_secs(3600));
    assert_eq!(cache.max_key_size, 1000);
    assert_eq!(cache.max_value_size, 50_000_000);
}

// ===== MACHINE LEARNING PIPELINE =====

#[test]
fn test_ml_pipeline() {
    trait DataPreprocessor<T, U> {
        type Error: std::fmt::Debug;
        fn preprocess(&self, input: T) -> Result<U, Self::Error>;
    }

    trait Model<T, U> {
        type Error: std::fmt::Debug;
        fn predict(&self, input: T) -> Result<U, Self::Error>;
    }

    struct TextPreprocessor;
    impl DataPreprocessor<String, Vec<f32>> for TextPreprocessor {
        type Error = String;
        fn preprocess(&self, _input: String) -> Result<Vec<f32>, Self::Error> {
            Ok(vec![1.0, 2.0, 3.0]) // Mock vectorization
        }
    }

    struct LinearModel;
    impl Model<Vec<f32>, f32> for LinearModel {
        type Error = String;
        fn predict(&self, _input: Vec<f32>) -> Result<f32, Self::Error> {
            Ok(0.85) // Mock prediction
        }
    }

    #[derive(TypeStateBuilder)]
    #[builder(build_method = "create_pipeline")]
    struct MLPipeline<'a, P, M, I, T, O, F, G>
    where
        P: DataPreprocessor<I, T>,
        M: Model<T, O>,
        F: Fn(&P::Error) + Send + Sync + 'a,
        G: Fn(&M::Error) + Send + Sync + 'a,
    {
        #[builder(required)]
        preprocessor: P,

        #[builder(required)]
        model: M,

        #[builder(required)]
        pipeline_name: String,

        #[builder(required)]
        preprocessing_error_handler: F,

        #[builder(required)]
        model_error_handler: G,

        #[builder(skip_setter, default = "PhantomData")]
        input_type: PhantomData<I>,

        #[builder(skip_setter, default = "PhantomData")]
        intermediate_type: PhantomData<T>,

        #[builder(skip_setter, default = "PhantomData")]
        output_type: PhantomData<O>,

        #[builder(skip_setter, default = "PhantomData")]
        lifetime_marker: PhantomData<&'a ()>,

        #[builder(default = "true")]
        enable_caching: bool,

        #[builder(default = "1000")]
        batch_size: usize,

        #[builder(default = "0.95")]
        confidence_threshold: f32,

        model_version: Option<String>,

        #[builder(skip_setter, default = "SystemTime::now()")]
        created_at: SystemTime,
    }

    let pipeline = MLPipeline::builder()
        .preprocessor(TextPreprocessor)
        .model(LinearModel)
        .pipeline_name("sentiment_analysis".to_string())
        .preprocessing_error_handler(|error: &String| {
            eprintln!("Preprocessing failed: {error}");
        })
        .model_error_handler(|error: &String| {
            eprintln!("Model prediction failed: {error}");
        })
        .batch_size(500)
        .confidence_threshold(0.9)
        .model_version(Some("v2.1.0".to_string()))
        .create_pipeline();

    assert_eq!(pipeline.pipeline_name, "sentiment_analysis");
    assert_eq!(pipeline.batch_size, 500);
    assert_eq!(pipeline.confidence_threshold, 0.9);
    assert!(pipeline.enable_caching);

    // Test unused trait methods - preprocess and predict
    let preprocess_result = pipeline.preprocessor.preprocess("test input".to_string());
    assert!(preprocess_result.is_ok());
    let features = preprocess_result.unwrap();
    assert_eq!(features, vec![1.0, 2.0, 3.0]);

    let predict_result = pipeline.model.predict(features);
    assert!(predict_result.is_ok());
    assert_eq!(predict_result.unwrap(), 0.85);

    // Test unused fields - preprocessor (tested above), model (tested above), preprocessing_error_handler, model_error_handler, model_version, created_at
    assert_eq!(pipeline.model_version, Some("v2.1.0".to_string()));
    assert!(pipeline.created_at <= SystemTime::now());

    // Test error handlers by calling them
    let preprocess_error = "Test preprocessing error".to_string();
    (pipeline.preprocessing_error_handler)(&preprocess_error);

    let model_error = "Test model error".to_string();
    (pipeline.model_error_handler)(&model_error);

    // Test PhantomData fields - zero-sized but we verify they're present
    assert_eq!(std::mem::size_of_val(&pipeline.input_type), 0);
    assert_eq!(std::mem::size_of_val(&pipeline.intermediate_type), 0);
    assert_eq!(std::mem::size_of_val(&pipeline.output_type), 0);
    assert_eq!(std::mem::size_of_val(&pipeline.lifetime_marker), 0);
}
