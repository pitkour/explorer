export default {
    formatUnixTimestamp(timestamp, locale = "pl-PL") {
        let date = new Date(timestamp);
        return date.toLocaleString(locale);
    }
};
