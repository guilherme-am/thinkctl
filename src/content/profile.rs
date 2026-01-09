pub const WHOAMI_PARAGRAPHS: [&str; 3] = [
    "DevOps & Platform Engineer · AI/MLOps tooling enthusiast · Cloud-native · Autonomous driving · 5G/6G.",
    "I build, break, and occasionally fix scalable systems. How? Not sure. Probably somewhere between wiring platforms, automating pipelines (and sometimes chaos), and building CLI/GUI tooling that makes developer life less boring.",
    "Across ~6+ years I’ve worked on production Kubernetes platforms (OpenShift + managed K8s), DevX environments, and reliability/observability systems — spanning autonomous vehicle teams, telco cloud/network platforms, and an early-stage SaaS where I touched backend, frontend, infra, costs, and agentic architecture.",
];

pub const HIGHLIGHTS: [(&str, &str, &str); 6] = [
    (
        "~~Automation~~ BreakingThings first",
        "I like turning messy constraints into something shippable: CLIs, GUIs, internal platforms, and automation that reduces toil.",
        "linux",
    ),
    (
        "Virtualization /Containers / OCI",
        "Containerization is my comfort zone: Docker, Podman/rootless, containerd, image pipelines, registries, and reproducible environments.",
        "docker",
    ),
    (
        "Kubernetes platforms",
        "I’ve lived inside EKS, GKE, OpenShift, on-prem clusters and managed K8s: Helm charts, GitOps (Argo CD), and operating real clusters under real pressure.",
        "kubernetes",
    ),
    (
        "Reliability & observability",
        "I enjoy troubleshooting: metrics that matter, dashboards that help, and systems that are debuggable at 3 AM.",
        "elasticsearch",
    ),
    (
        "AI/MLOps tooling",
        "I’m into MLOps pipelines and agentic tooling that automates the boring parts and makes infra smarter.",
        "kubeflow",
    ),
    (
        "Writing & sharing",
        "If I’m not building, I’m documenting: system notes, postmortems, and reusable “service units” under /etc/thoughts.",
        "markdown",
    ),
];

pub const SKILLS_PLATFORM: [(&str, &str, &str, Option<&str>); 6] = [
    ("kubernetes", "Kubernetes", "blue", None),
    ("openshift", "OpenShift", "red", None),
    (
        "cncf",
        "CNCF",
        "blue",
        Some("https://www.cncf.io/wp-content/uploads/2023/04/cncf-main-site-logo.svg"),
    ),
    ("aws", "AWS", "blue", None),
    ("gcp", "GCP", "blue", None),
    ("azure", "Azure", "blue", None),
];

pub const SKILLS_CONTAINERS: [(&str, &str, &str, Option<&str>); 3] = [
    ("docker", "Docker", "blue", None),
    (
        "podman",
        "Podman",
        "red",
        Some("https://podman.io/logos/optimized/podman-3-logo-266w-253h.webp"),
    ),
    (
        "helm",
        "Helm",
        "blue",
        Some("https://helm.sh/pt/img/helm.svg"),
    ),
];

pub const SKILLS_IAC_CICD: [(&str, &str, &str, Option<&str>); 4] = [
    ("terraform", "Terraform", "purple", None),
    ("ansible", "Ansible", "purple", None),
    ("githubactions", "GitHub Actions", "purple", None),
    ("gitlab", "GitLab CI", "purple", None),
];

pub const SKILLS_OBSERVABILITY: [(&str, &str, &str, Option<&str>); 9] = [
    ("prometheus", "Prometheus", "orange", None),
    ("grafana", "Grafana", "orange", None),
    (
        "opentelemetry",
        "OpenTelemetry",
        "orange",
        Some("https://opentelemetry.io/img/logos/opentelemetry-horizontal-color.svg"),
    ),
    (
        "elasticsearch",
        "Elasticsearch",
        "orange",
        Some("https://cdn.optimizely.com/img/18132920325/bb267dd0fde04a47bf59cb3989c9512b.png"),
    ),
    ("kibana", "Kibana", "orange", None),
    ("logstash", "Logstash", "orange", None),
    (
        "fluentd",
        "Fluentd",
        "orange",
        Some("https://www.fluentd.org/images/miscellany/fluentd-logo.png"),
    ),
    (
        "jaeger",
        "Jaeger",
        "orange",
        Some("https://www.jaegertracing.io/img/jaeger-icon-color.png"),
    ),
    (
        "opensearch",
        "OpenSearch",
        "orange",
        Some("https://opensearch.org/wp-content/uploads/2024/08/opensearch_logo_default.svg"),
    ),
];

pub const SKILLS_AI_MLOPS: [(&str, &str, &str, Option<&str>); 4] = [
    ("kubeflow", "Kubeflow", "orange", None),
    ("mlflow", "MLflow", "blue", None),
    ("pytorch", "PyTorch", "red", None),
    ("huggingface", "Hugging Face", "orange", None),
];

pub const SKILLS_LANGUAGES: [(&str, &str, &str, Option<&str>); 6] = [
    ("python", "Python", "green", None),
    ("ts", "TypeScript", "blue", None),
    ("rust", "Rust", "orange", None),
    ("go", "Go", "blue", None),
    ("bash", "Bash", "green", None),
    ("js", "JavaScript", "blue", None),
];

pub const SKILLS_DATABASES: [(&str, &str, &str, Option<&str>); 4] = [
    ("postgres", "PostgreSQL", "blue", None),
    ("mongodb", "MongoDB", "green", None),
    ("neo4j", "Neo4j", "green", None),
    ("dynamodb", "DynamoDB", "blue", None),
];

pub const CERTS: [(&str, Option<&str>); 7] = [
    (
        "Machine Learning Specialization — Stanford University",
        Some("https://skillicons.dev/icons?i=tensorflow&theme=dark"),
    ),
    (
        "AWS Cloud Consultant Specialist; AWS CloudOps Engineer",
        Some("https://skillicons.dev/icons?i=aws&theme=dark"),
    ),
    (
        "Google Cloud — Cloud Architect (training)",
        Some("https://skillicons.dev/icons?i=gcp&theme=dark"),
    ),
    (
        "MLOps Specialization — Duke University",
        Some(
            "https://d3njjcbhbojbot.cloudfront.net/api/utilities/v1/imageproxy/https://coursera-university-assets.s3.amazonaws.com/8c/3fea3c61b899a0e79075dc8c96195d/dukewidelogo2.png?auto=format%2Ccompress&dpr=2&h=45",
        ),
    ),
    (
        "CKA / DCA / Linux Sysadmin — The Linux Foundation",
        Some("https://skillicons.dev/icons?i=linux&theme=dark"),
    ),
    (
        "OpenShift Administration",
        Some("https://skillicons.dev/icons?i=openshift&theme=dark"),
    ),
    (
        "Nokia Bell Labs — 5G Distributed Cloud Professional (MEC / Edge / O-RAN)",
        None,
    ),
];
