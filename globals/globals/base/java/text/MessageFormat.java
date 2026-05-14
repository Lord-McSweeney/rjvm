package java.text;

public class MessageFormat extends Format {
    public static String format(String pattern, Object... arguments) {
        // TODO implement properly- this is good enough for now
        String result = pattern;

        for (int i = 0; i < arguments.length; i ++) {
            String argument = String.valueOf(arguments[i]);
            String search = "{" + i + '}';
            String[] split = result.split(search);

            StringBuilder combined = new StringBuilder();
            for (int j = 0; j < split.length; j ++) {
                combined.append(split[j]);
                if (j != split.length - 1) {
                    combined.append(argument);
                }
            }
            result = combined.toString();
        }
        return result;
    }
}
