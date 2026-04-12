package java.util.regex;

public final class Pattern {
    private rjvm.libs.com.google.re2j.Pattern pattern;

    Pattern(rjvm.libs.com.google.re2j.Pattern pattern) {
        this.pattern = pattern;
    }

    public static Pattern compile(String regex) {
        return new Pattern(rjvm.libs.com.google.re2j.Pattern.compile(regex));
    }

    public Matcher matcher(CharSequence test) {
        return new Matcher(this.pattern.matcher(test));
    }
}
