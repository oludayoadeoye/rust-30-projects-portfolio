import os
import subprocess
import re

projects = {
    "actix-web": ["restaurant-res", "fitness-tracker", "job-board", "healthcare-mgmt", "smart-inventory"]
}

BASE_DIR = "/Users/champion/RustPractise"
MMDC_PATH = "/usr/local/bin/mmdc"

def get_context(path):
    models_path = os.path.join(path, "src/models.rs")
    structs = []
    if os.path.exists(models_path):
        with open(models_path, 'r') as f:
            structs = re.findall(r'pub struct (\w+)', f.read())
    return structs

def generate_mmd(name, structs):
    struct_str = "\n".join([f"    class {s}" for s in structs])
    return {
        "architecture": f"graph TD\n    User((User)) -->|REST| API[API: {name}]\n    API -->|SQLx| DB[(Postgres)]",
        "class": f"classDiagram\n{struct_str}\n    class DB {{ +created_at }}\n",
        "flowchart": "graph LR\n    Req[Request] --> Val{Validate} --> Logic[Logic] --> DB[(DB)] --> Res[Response]",
        "sequence": "sequenceDiagram\n    User->>API: Request\n    API->>DB: Query\n    DB-->>API: Data\n    API-->>User: JSON",
        "mindmap": f"mindmap\n    root(({name}))\n        Stack\n            Rust\n            SQLx\n            Docker\n        Entities\n            {') ) ('.join(structs)}",
        "timeline": "timeline\n    title Lifecycle\n    Design : Plan : Code : Test"
    }

def process():
    for fw, names in projects.items():
        for name in names:
            print(f">>> Processing: {name}")
            path = os.path.join(BASE_DIR, fw, name)
            asset_dir = os.path.join(path, "assets/diagrams")
            os.makedirs(asset_dir, exist_ok=True)
            
            structs = get_context(path)
            diagrams = generate_mmd(name, structs)
            
            for dtype, content in diagrams.items():
                mmd_file = os.path.join(asset_dir, f"{dtype}.mmd")
                with open(mmd_file, "w") as f:
                    f.write(content)
                
                for ext in ["png", "svg", "pdf"]:
                    out_file = os.path.join(asset_dir, f"{dtype}.{ext}")
                    subprocess.run([MMDC_PATH, "-i", mmd_file, "-o", out_file], capture_output=True)
                print(f"  Generated {dtype}")

if __name__ == "__main__":
    process()
