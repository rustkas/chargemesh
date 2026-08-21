//! Simulator integration tests

use chargemesh_simulator::*;
use chargemesh_simulator::core::*;

#[test]
fn test_battery_simulation() {
    use chargemesh_simulator::ev::battery::Battery;

    let mut battery = Battery::new(75000, 20);
    assert_eq!(battery.soc(), 20);

    battery.charge(10000).unwrap();
    assert!(battery.soc() > 20);

    battery.discharge(5000).unwrap();
    assert!(battery.get_temperature() > 20.0);
}

#[test]
fn test_scenario_creation() {
    let scenario = Scenarios::normal_session();
    assert_eq!(scenario.name, "Normal Charging Session");
    assert!(!scenario.steps.is_empty());

    let scenario = Scenarios::network_failure();
    assert_eq!(scenario.name, "Network Failure");
    assert!(!scenario.faults.is_empty());

    let scenario = Scenarios::auth_failure();
    assert_eq!(scenario.name, "Authorization Failure");
}

#[test]
fn test_fault_injection() {
    use chargemesh_simulator::faults::*;

    let injector = FaultInjector::new();
    let fault = FaultInjection {
        fault_type: FaultType::NetworkDisconnect,
        probability: 1.0,
        duration: chrono::Duration::seconds(5),
        condition: None,
        parameters: serde_json::json!({}),
    };

    let result = injector.inject_fault(fault);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_scenario_runner() {
    let runner = ScenarioRunner::new();
    let scenario = Scenarios::normal_session();

    let result = runner.run(&scenario).await;
    assert!(result.is_ok());
    assert_eq!(runner.status().await, SimulationStatus::Completed);
}