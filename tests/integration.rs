#[cfg(test)]
mod tests {
    use slack_emojify::Emojify;

    #[test]
    fn test_emojify() {
        assert_eq!(
            ":hiking_boot: :anger::canned_food: :wavy_dash: :motorway: I kicked the can down the road on my other two in-progress tasks.".emojify(),
            "🥾 💢🥫 〰 🛣 I kicked the can down the road on my other two in-progress tasks."
        );
    }
}
