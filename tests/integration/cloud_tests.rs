//! Cloud platform tests

use chargemesh_cloud::*;

#[tokio::test]
async fn test_cloud_platform() {
    let config = PlatformConfig {
        name: "test".to_string(),
        version: "1.0.0".to_string(),
        environment: Environment::Development,
        api_port: 8080,
        database_url: "postgres://localhost".to_string(),
        redis_url: "redis://localhost".to_string(),
        jwt_secret: "test-secret".to_string(),
    };

    let platform = CloudPlatform::new(config).await.unwrap();
    platform.start().await.unwrap();

    // Check status
    let status = platform.get_status().await;
    assert_eq!(status, PlatformStatus::Running);

    platform.stop().await.unwrap();
    let status = platform.get_status().await;
    assert_eq!(status, PlatformStatus::Stopped);
}

#[tokio::test]
async fn test_tenant_manager() {
    let manager = tenant::TenantManager::new();

    let tenant = tenant::Tenant {
        id: "tenant-1".to_string(),
        name: "Test Tenant".to_string(),
        tier: CloudTier::Pro,
        subscription_id: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        status: tenant::TenantStatus::Active,
        quota: tenant::Quota {
            max_stations: 100,
            max_evses: 200,
            max_sessions: 1000,
            storage_gb: 50,
            api_calls_per_month: 10000,
            max_users: 10,
            retention_days: 30,
        },
    };

    manager.create_tenant(tenant).await.unwrap();
    let retrieved = manager.get_tenant("tenant-1").await.unwrap();
    assert_eq!(retrieved.name, "Test Tenant");
}

#[tokio::test]
async fn test_billing_manager() {
    let manager = billing::BillingManager::new();

    let subscription = billing::Subscription {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id: "tenant-1".to_string(),
        tier: CloudTier::Pro,
        start_date: chrono::Utc::now(),
        end_date: None,
        status: billing::SubscriptionStatus::Active,
        price: 99.0,
        currency: "USD".to_string(),
        billing_cycle: billing::BillingCycle::Monthly,
    };

    manager.create_subscription(subscription).await.unwrap();
    let retrieved = manager.get_subscription("tenant-1").await.unwrap();
    assert_eq!(retrieved.tier, CloudTier::Pro);
}