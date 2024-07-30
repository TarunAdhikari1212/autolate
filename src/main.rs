use aws_sdk_translate as translate;
use aws_config::meta::region::RegionProviderChain;
use std::io;
use tokio;

#[tokio::main]
async fn main() ->Result<(), translate::Error> {

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = aws_sdk_translate::Client::new(&config);

    let languages = [
        ("Afrikaans", "af"),
        ("Albanian", "sq"),
        ("Amharic", "am"),
        ("Arabic", "ar"),
        ("Armenian", "hy"),
        ("Azerbaijani", "az"),
        ("Bengali", "bn"),
        ("Bosnian", "bs"),
        ("Bulgarian", "bg"),
        ("Catalan", "ca"),
        ("Chinese (Simplified)", "zh"),
        ("Chinese (Traditional)", "zh-TW"),
        ("Croatian", "hr"),
        ("Czech", "cs"),
        ("Danish", "da"),
        ("Dari", "fa-AF"),
        ("Dutch", "nl"),
        ("English", "en"),
        ("Estonian", "et"),
        ("Farsi (Persian)", "fa"),
        ("Filipino, Tagalog", "tl"),
        ("Finnish", "fi"),
        ("French", "fr"),
        ("French (Canada)", "fr-CA"),
        ("Georgian", "ka"),
        ("German", "de"),
        ("Greek", "el"),
        ("Gujarati", "gu"),
        ("Haitian Creole", "ht"),
        ("Hausa", "ha"),
        ("Hebrew", "he"),
        ("Hindi", "hi"),
        ("Hungarian", "hu"),
        ("Icelandic", "is"),
        ("Indonesian", "id"),
        ("Irish", "ga"),
        ("Italian", "it"),
        ("Japanese", "ja"),
        ("Kannada", "kn"),
        ("Kazakh", "kk"),
        ("Korean", "ko"),
        ("Latvian", "lv"),
        ("Lithuanian", "lt"),
        ("Macedonian", "mk"),
        ("Malay", "ms"),
        ("Malayalam", "ml"),
        ("Maltese", "mt"),
        ("Mandarin", "zh-CN"),
        ("Marathi", "mr"),
        ("Mongolian", "mn"),
        ("Nepali", "ne"),
        ("Norwegian", "no"),
        ("Pashto", "ps"),
        ("Persian", "fa"),
        ("Polish", "pl"),
        ("Portuguese", "pt"),
        ("Punjabi", "pa"),
        ("Romanian", "ro"),
        ("Russian", "ru"),
        ("Serbian", "sr"),
        ("Sinhala", "si"),
        ("Slovak", "sk"),
        ("Slovenian", "sl"),
        ("Somali", "so"),
        ("Spanish", "es"),
        ("Swahili", "sw"),
        ("Swedish", "sv"),
        ("Tagalog", "tl"),
        ("Tamil", "ta"),
        ("Telugu", "te"),
        ("Thai", "th"),
        ("Turkish", "tr"),
        ("Ukrainian", "uk"),
        ("Urdu", "ur"),
        ("Uzbek", "uz"),
        ("Vietnamese", "vi"),
        ("Welsh", "cy")
    ];
    // println!("Supported Languages:");
    // for lang in languages {
    //     println!("{}",lang.0);
    // }

    let mut src_lang = String::new();
    let mut tar_lang = String::new();
    let mut src_text = String::new();

    println!("Enter source language:");
    io::stdin().read_line(&mut src_lang).expect("Failed to read line");
    println!("Enter target language:");
    io::stdin().read_line(&mut tar_lang).expect("Failed to read line");
    println!("Enter text to translate:");
    io::stdin().read_line(&mut src_text).expect("Failed to read line");

    // for (_,short) in languages.iter().enumerate() {
    if languages.iter().any(|&(lang, _)| lang == src_lang.trim()) && languages.iter().any(|&(lang, _)| lang == tar_lang.trim()) 
    {
        let source = languages
            .iter()
            .find(|&&(lang, _)| lang == src_lang.trim()).unwrap().1;

        let target = languages
            .iter()
            .find(|&&(lang, _)| lang == tar_lang.trim()).unwrap().1;

        println!("Source language: {}", source);
        println!("Target language: {}", target);
        let response = client.translate_text()
            .text(src_text.trim())
            .source_language_code(source)
            .target_language_code(target)
            .send().await?;

        println!("Translated text: {}", response.translated_text());
    } else {
        println!("Invalid language codes or text");
    }

    Ok(())
}



// Python version:
// import json
// import boto3
// def lambda_handler(event, context):
//     print(event)
//     translate_client=boto3.client('translate')
//     sl = event['arguments']['input']['source_language']
//     tl = event['arguments']['input']['target_language']
//     if sl in languages.keys():
//         source = languages[sl]
//     if tl in languages.keys():
//         target = languages[tl]
//     try:
//         response=translate_client.translate_text(Text=event['arguments']['input']['text'],SourceLanguageCode=source,TargetLanguageCode=target)
//         translated_text=response['TranslatedText']
//         return {
//             'statusCode': 200,
//             'body': json.dumps(translated_text)
//         }
//     except:
//         return{
//             'statusCode': 400,
//             'body': json.dumps("Error")
//         }