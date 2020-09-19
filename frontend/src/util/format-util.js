export default {
    formatUnixTimestamp(timestamp, locale = "pl-PL") {
        let date = new Date(timestamp);
        return date.toLocaleString(locale);
    },

    formatEnum(enumValue) {
        let first = true;
        let result = "";
        let split = enumValue.split("_");
        for (const part of split) {
            if (first) {
                first = false;
            } else {
                result += " ";
            }
            let prefix = part.substring(0, 1);
            let residue = part.substring(1);
            result += prefix + residue.toLowerCase();
        }
        return result;
    },

    formatBoolean(value) {
        return value ? "Yes" : "No";
    }
};
