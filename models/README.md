# Models Directory

This directory contains the ONNX models used for generating embeddings.

## Required Files

You need to download the following files for the `all-MiniLM-L6-v2` model:

1. `all-MiniLM-L6-v2.onnx` - The ONNX model file
2. `tokenizer.json` - The tokenizer configuration

## Download Instructions

### Option 1: From HuggingFace (Recommended)

Visit the model page: https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2

Download the following files to this directory:
- `model.onnx` (rename to `all-MiniLM-L6-v2.onnx`)
- `tokenizer.json`

### Option 2: Using Python

If you have Python and transformers installed:

```python
from optimum.onnxruntime import ORTModelForFeatureExtraction
from transformers import AutoTokenizer

model_name = "sentence-transformers/all-MiniLM-L6-v2"

# Export to ONNX
model = ORTModelForFeatureExtraction.from_pretrained(model_name, export=True)
model.save_pretrained("./models")

# Save tokenizer
tokenizer = AutoTokenizer.from_pretrained(model_name)
tokenizer.save_pretrained("./models")
```

### Option 3: Using optimum-cli

```bash
pip install optimum[onnxruntime]

optimum-cli export onnx \
  --model sentence-transformers/all-MiniLM-L6-v2 \
  --task feature-extraction \
  ./models
```

## Model Details

- **Name**: all-MiniLM-L6-v2
- **Dimensions**: 384
- **Max Sequence Length**: 256
- **Size**: ~23MB (ONNX)
- **Speed**: Fast CPU inference
- **Use Case**: Semantic similarity, search

## Verification

After downloading, you should have:
```
models/
├── README.md
├── all-MiniLM-L6-v2.onnx
└── tokenizer.json
```

The model files are gitignored and must be downloaded manually.
