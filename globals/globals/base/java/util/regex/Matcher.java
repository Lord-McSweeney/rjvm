package java.util.regex;

public final class Matcher implements MatchResult {
    private rjvm.libs.com.google.re2j.Matcher matcher;

    Matcher(rjvm.libs.com.google.re2j.Matcher matcher) {
        this.matcher = matcher;
    }

    public boolean matches() {
        return this.matcher.matches();
    }

    public String group() {
        return this.matcher.group();
    }

    public String group(int index) {
        return this.matcher.group(index);
    }
}
