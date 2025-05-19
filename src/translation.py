#!/usr/bin/env python3
import sys
import os
from transformers import MarianMTModel, MarianTokenizer

def translate_text(input_file, output_file, target_language):
    language_to_model = { #english -> target language
        "es": "Helsinki-NLP/opus-mt-en-es", #spanish
        "fr": "Helsinki-NLP/opus-mt-en-fr", #french
        "de": "Helsinki-NLP/opus-mt-en-de", #german
        "ru": "Helsinki-NLP/opus-mt-en-ru", #russian
        "zh": "Helsinki-NLP/opus-mt-en-zh", #chinese
    }
    
    #language mapping defaulting to french
    model_name = language_to_model.get(target_language, "Helsinki-NLP/opus-mt-en-fr")
    
    try:
        # Load model and tokenizer
        tokenizer = MarianTokenizer.from_pretrained(model_name)
        model = MarianMTModel.from_pretrained(model_name)
        
        # Read input file
        with open(input_file, 'r', encoding='utf-8') as f:
            text = f.read()
        
        # Tokenize and translate
        inputs = tokenizer(text, return_tensors="pt", padding=True)
        translated = model.generate(**inputs)
        translated_text = tokenizer.batch_decode(translated, skip_special_tokens=True)[0]
        
        # Write to output file
        with open(output_file, 'w', encoding='utf-8') as f:
            f.write(translated_text)
            
        return True
    except Exception as e:
        print(f"Error during translation: {str(e)}")
        return False

if __name__ == "__main__":
    if len(sys.argv) != 4:
        print("Error - expected: translator.py <input_file> <output_file> <target_language>")
        sys.exit(1)
    
    input_file = sys.argv[1]
    output_file = sys.argv[2]
    target_language = sys.argv[3]
    
    success = translate_text(input_file, output_file, target_language)
    sys.exit(0 if success else 1)