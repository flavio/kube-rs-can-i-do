use k8s_openapi::api::authorization::v1::{
    ResourceAttributes, SubjectAccessReview, SubjectAccessReviewSpec, SubjectAccessReviewStatus,
};
use kube::{Api, Client, api::PostParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;

    let sar_api: Api<SubjectAccessReview> = Api::all(client);

    let sar = SubjectAccessReview {
        spec: SubjectAccessReviewSpec {
            // system:serviceaccount:<namespace>:<serviceaccountname>
            user: Some("system:serviceaccount:default:demo1".to_string()),
            resource_attributes: Some(ResourceAttributes {
                namespace: Some("default".to_string()),
                verb: Some("list".to_string()),
                resource: Some("secrets".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    };

    let response = sar_api.create(&PostParams::default(), &sar).await?;
    println!("SubjectAccessReview response: {:?}", response);

    match response.status {
        Some(SubjectAccessReviewStatus { allowed: true, .. }) => {
            println!("✅ ServiceAccount is allowed");
        }
        Some(SubjectAccessReviewStatus { allowed: false, .. }) => {
            println!("❌ ServiceAccount is NOT allowed");
        }
        _ => {
            println!("⚠️ Could not determine access");
        }
    }

    Ok(())
}
