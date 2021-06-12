public class Inheritance {
	static int FIVE = 5;
	int five = 5;

	static class Inner extends Inheritance {
		static int SIX = 6;
		int six = 6;
	}

	static class InnerDeeper extends Inner {}
}
