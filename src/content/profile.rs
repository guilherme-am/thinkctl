pub const WHOAMI_PARAGRAPHS: [&str; 3] = [
    "Platform engineer. Years of IaC and DevEx. These days most of my focus is productionizing agents: from definition to tools, least privilege, traces, evals, and the datasets that make the next loop less lucky.",
    "I still build, break, and occasionally fix scalable systems. Kubernetes platforms, GitOps, CLIs, internal tooling. The agent work sits on that foundation instead of replacing it.",
    "Across ~6+ years I have worked on production Kubernetes (OpenShift and managed K8s), developer platforms, and reliability/observability, spanning autonomous vehicle teams, telco cloud, and an early-stage SaaS where I touched backend, frontend, infra, costs, and agentic architecture.",
];

pub const HIGHLIGHTS: [(&str, &str, &str, Option<&str>, bool); 6] = [
    (
        "Productionizing agents",
        "This is the main work now. Agent definition (DeepAgents, Google ADK, Jev), SDK wiring (LangChain, Claude, OpenAI, A2A), least-privilege access, traces and evals (Langfuse, LangSmith), then refinement datasets so the system can get more reliable over time.",
        "py",
        Some("https://cdn.jsdelivr.net/npm/simple-icons@v15/icons/langchain.svg"),
        true,
    ),
    (
        "~~Automation~~ BreakingThings first",
        "I like turning messy constraints into something shippable: CLIs, GUIs, internal platforms, and automation that reduces toil.",
        "linux",
        None,
        false,
    ),
    (
        "IaC & DevEx",
        "Terraform, Ansible, GitHub Actions, GitLab CI. I am still an infra-as-code person. Agent systems that cannot be provisioned, reviewed, and torn down like the rest of the platform will not last.",
        "terraform",
        None,
        false,
    ),
    (
        "Kubernetes platforms",
        "I have lived inside EKS, GKE, OpenShift, on-prem clusters and managed K8s: Helm charts, GitOps (Argo CD), and operating real clusters under real pressure.",
        "kubernetes",
        None,
        false,
    ),
    (
        "Reliability & observability",
        "I enjoy troubleshooting: metrics that matter, dashboards that help, and systems that are debuggable at 3 AM. Agents need that stack too, not only the LLM traces.",
        "elasticsearch",
        None,
        false,
    ),
    (
        "Writing & sharing",
        "If I am not building, I am documenting: system notes, postmortems, and reusable service units under /etc/thoughts.",
        "markdown",
        None,
        false,
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
    (
        "kibana",
        "Kibana",
        "orange",
        Some(
            "https://e7.pngegg.com/pngimages/131/974/png-clipart-kibana-elasticsearch-scalable-graphics-logo-logstash-chess24-angle-text.png",
        ),
    ),
    (
        "logstash",
        "Logstash",
        "orange",
        Some(
            "https://assets.streamlinehq.com/image/private/w_300,h_300,ar_1/f_auto/v1/icons/4/logstash-fo8a4b773cwws2lq60rlh.png/logstash-4s2efyjt56l53mlg59jrk.png?_a=DATAg1fmZAA0",
        ),
    ),
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

pub const SKILLS_AGENTS: [(&str, &str, &str, Option<&str>); 8] = [
    (
        "langchain",
        "LangChain",
        "green",
        Some("https://cdn.jsdelivr.net/npm/simple-icons@v15/icons/langchain.svg"),
    ),
    (
        "deepagents",
        "DeepAgents",
        "green",
        Some("https://cdn.jsdelivr.net/npm/simple-icons@v15/icons/langchain.svg"),
    ),
    ("gcp", "Google ADK", "blue", None),
    (
        "a2a",
        "A2A",
        "blue",
        Some("https://cdn.jsdelivr.net/npm/simple-icons@v15/icons/google.svg"),
    ),
    (
        "anthropic",
        "Claude SDK",
        "orange",
        Some("https://cdn.jsdelivr.net/npm/simple-icons@v15/icons/anthropic.svg"),
    ),
    (
        "openai",
        "OpenAI SDK",
        "green",
        Some("https://cdn.jsdelivr.net/npm/simple-icons@v15/icons/openai.svg"),
    ),
    (
        "langfuse",
        "Langfuse",
        "orange",
        Some("https://cdn.jsdelivr.net/gh/langfuse/langfuse@main/web/public/icon256.png"),
    ),
    (
        "langsmith",
        "LangSmith",
        "orange",
        Some("https://cdn.jsdelivr.net/npm/simple-icons@v15/icons/langchain.svg"),
    ),
];

pub const SKILLS_AI_MLOPS: [(&str, &str, &str, Option<&str>); 4] = [
    (
        "kubeflow",
        "Kubeflow",
        "orange",
        Some("https://raw.githubusercontent.com/kubeflow/website/master/static/images/logo.svg"),
    ),
    ("mlflow", "MLflow", "blue", None),
    ("pytorch", "PyTorch", "red", None),
    (
        "huggingface",
        "Hugging Face",
        "orange",
        Some("https://huggingface.co/front/assets/huggingface_logo-noborder.svg"),
    ),
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
