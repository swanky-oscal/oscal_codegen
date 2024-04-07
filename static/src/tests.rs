#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    static DOC_ROOT: &str = "../../../oscal";
    static FEDRAMP_AUTOMATION: &str = "fedramp-automation/dist/content";
    static OSCAL_CONTENT: &str = "oscal-content/examples";
    static NIST_CONTENT: &str = "oscal-content/nist.gov";

    fn read_doc(root: &str, path: &str) -> Result<OscalDocument, String> {
        let json = fs::read_to_string(format!("{}/{}/{}", DOC_ROOT, root, path))
            .map_err(|e| e.to_string())?;

        serde_json::from_str::<OscalDocument>(&json).map_err(|e| e.to_string())
    }

    #[test]
    fn test_fedramp_automation_sap() {
        let result = read_doc(
            FEDRAMP_AUTOMATION,
            "rev5/templates/sap/json/FedRAMP-SAP-OSCAL-Template.json",
        );

        assert!(result.is_ok());
        let result = result.unwrap();

        let OscalDocumentType::AssessmentPlan(ap) = result.document else {
            panic!("Not an AssessmentPlan");
        };

        let doc = OscalDocument {
            schema: None,
            document: OscalDocumentType::AssessmentPlan(ap),
        };

        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_sap() {
        let result = read_doc(OSCAL_CONTENT, "ap/json/ifa_assessment-plan-example.json");

        assert!(result.is_ok());
        let result = result.unwrap();

        let OscalDocumentType::AssessmentPlan(ap) = result.document else {
            panic!("Not an AssessmentPlan");
        };

        let doc = OscalDocument {
            schema: None,
            document: OscalDocumentType::AssessmentPlan(ap),
        };

        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fedramp_automation_sar() {
        let result = read_doc(
            FEDRAMP_AUTOMATION,
            "rev5/templates/sar/json/FedRAMP-SAR-OSCAL-Template.json",
        );

        assert!(result.is_ok());
        let result = result.unwrap();
        let OscalDocumentType::AssessmentResults(ar) = result.document else {
            panic!("Not an AssessmentResult");
        };
        let doc = OscalDocument {
            schema: None,
            document: OscalDocumentType::AssessmentResults(ar),
        };

        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_sar() {
        let result = read_doc(OSCAL_CONTENT, "ar/json/ifa_assessment-results-example.json");

        assert!(result.is_ok());
        let result = result.unwrap();
        let OscalDocumentType::AssessmentResults(ar) = result.document else {
            panic!("Not an AssessmentResult");
        };
        let doc = OscalDocument {
            schema: None,
            document: OscalDocumentType::AssessmentResults(ar),
        };

        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fedramp_automation_catalog() {
        let result = read_doc(
            FEDRAMP_AUTOMATION,
            "rev5/baselines/json/FedRAMP_rev5_HIGH-baseline-resolved-profile_catalog.json",
        );

        assert!(result.is_ok());
        let result = result.unwrap();
        let OscalDocumentType::Catalog(catalog) = result.document else {
            panic!("Not an Catalog");
        };
        let doc = OscalDocument {
            schema: None,
            document: OscalDocumentType::Catalog(catalog),
        };

        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_catalog() {
        let result = read_doc(OSCAL_CONTENT, "catalog/json/basic-catalog.json");

        assert!(result.is_ok());
        let result = result.unwrap();
        let OscalDocumentType::Catalog(catalog) = result.document else {
            panic!("Not an Catalog");
        };
        let doc = OscalDocument {
            schema: None,
            document: OscalDocumentType::Catalog(catalog),
        };

        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_nist_catalog() {
        let result = read_doc(
            NIST_CONTENT,
            "SP800-53/rev5/json/NIST_SP-800-53_rev5_catalog.json",
        );

        assert!(result.is_ok());
        let result = result.unwrap();
        let OscalDocumentType::Catalog(catalog) = result.document else {
            panic!("Not an Catalog");
        };
        let doc = OscalDocument {
            schema: None,
            document: OscalDocumentType::Catalog(catalog),
        };

        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_component_definition_component() {
        let result = read_doc(
            OSCAL_CONTENT,
            "component-definition/json/example-component.json",
        );

        assert!(result.is_ok());
        let result = result.unwrap();
        let OscalDocumentType::ComponentDefinition(component) = result.document else {
            panic!("Not an ComponentDefinition");
        };
        let doc = OscalDocument {
            schema: None,
            document: OscalDocumentType::ComponentDefinition(component),
        };

        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_component_definition_component_definition() {
        let result = read_doc(
            OSCAL_CONTENT,
            "component-definition/json/example-component-definition.json",
        );

        assert!(result.is_ok());
        let result = result.unwrap();
        let OscalDocumentType::ComponentDefinition(component) = result.document else {
            panic!("Not an ComponentDefinition");
        };
        let doc = OscalDocument {
            schema: None,
            document: OscalDocumentType::ComponentDefinition(component),
        };

        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fedramp_automation_poam() {
        let result = read_doc(
            FEDRAMP_AUTOMATION,
            "rev5/templates/poam/json/FedRAMP-POAM-OSCAL-Template.json",
        );

        assert!(result.is_ok());
        let result = result.unwrap();
        let OscalDocumentType::PlanOfActionAndMilestones(poam) = result.document else {
            panic!("Not an PlanOfActionAndMilestones");
        };
        let doc = OscalDocument {
            schema: None,
            document: OscalDocumentType::PlanOfActionAndMilestones(poam),
        };

        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_poam() {
        let result = read_doc(
            OSCAL_CONTENT,
            "poam/json/ifa_plan-of-action-and-milestones.json",
        );

        assert!(result.is_ok());
        let result = result.unwrap();
        let OscalDocumentType::PlanOfActionAndMilestones(poam) = result.document else {
            panic!("Not an PlanOfActionAndMilestones");
        };
        let doc = OscalDocument {
            schema: None,
            document: OscalDocumentType::PlanOfActionAndMilestones(poam),
        };

        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_fedramp_automation_ssp() {
        let result = read_doc(
            FEDRAMP_AUTOMATION,
            "rev5/templates/ssp/json/FedRAMP-SSP-OSCAL-Template.json",
        );

        assert!(result.is_ok());
        let result = result.unwrap();
        let OscalDocumentType::SystemSecurityPlan(ssp) = result.document else {
            panic!("Not an SystemSecurityPlan");
        };
        let doc = OscalDocument {
            schema: None,
            document: OscalDocumentType::SystemSecurityPlan(ssp),
        };

        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_ssp() {
        let result = read_doc(OSCAL_CONTENT, "ssp/json/ssp-example.json");

        assert!(result.is_ok());
        let result = result.unwrap();
        let OscalDocumentType::SystemSecurityPlan(ssp) = result.document else {
            panic!("Not an SystemSecurityPlan");
        };
        let doc = OscalDocument {
            schema: None,
            document: OscalDocumentType::SystemSecurityPlan(ssp),
        };

        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_ssp_oscal_leveraging() {
        let result = read_doc(OSCAL_CONTENT, "ssp/json/oscal_leveraging-example_ssp.json");

        assert!(result.is_ok());
        let result = result.unwrap();
        let OscalDocumentType::SystemSecurityPlan(ssp) = result.document else {
            panic!("Not an SystemSecurityPlan");
        };
        let doc = OscalDocument {
            schema: None,
            document: OscalDocumentType::SystemSecurityPlan(ssp),
        };

        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_ssp_oscal_leveraged() {
        let result = read_doc(OSCAL_CONTENT, "ssp/json/oscal_leveraged-example_ssp.json");

        assert!(result.is_ok());
        let result = result.unwrap();
        let OscalDocumentType::SystemSecurityPlan(ssp) = result.document else {
            panic!("Not an SystemSecurityPlan");
        };
        let doc = OscalDocument {
            schema: None,
            document: OscalDocumentType::SystemSecurityPlan(ssp),
        };

        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_oscal_content_ssp_ifa() {
        let result = read_doc(OSCAL_CONTENT, "ssp/json/ifa_ssp-example.json");

        assert!(result.is_ok());
        let result = result.unwrap();
        let OscalDocumentType::SystemSecurityPlan(ssp) = result.document else {
            panic!("Not an SystemSecurityPlan");
        };
        let doc = OscalDocument {
            schema: None,
            document: OscalDocumentType::SystemSecurityPlan(ssp),
        };

        let result = serde_json::to_string_pretty(&doc);
        assert!(result.is_ok());
    }
}
