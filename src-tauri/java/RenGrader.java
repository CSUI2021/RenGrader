import java.io.File;
import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.io.InputStream;
import java.io.PrintStream;
import java.io.PrintWriter;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.Comparator;
import java.util.concurrent.Callable;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.FutureTask;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.TimeoutException;

public class RenGrader {
	private static Integer timeout = {{ TIMEOUT }};
	private static String testCaseDir = "{{ TEST_CASE_DIR }}";
	private static ExecutorService executor = Executors.newSingleThreadExecutor();
	private static PrintStream origOut = System.out;
	private static InputStream origIn = System.in;

	public static String getBaseName(String fileName) {
		int index = fileName.lastIndexOf('.');
		if (index == -1) {
			return fileName;
		} else {
			return fileName.substring(0, index);
		}
	}

	public static String runTest(File input, File output, File myOutput) {
		InputStream in;
		PrintStream out;
		try {
			in = new FileInputStream(input);
			out = new PrintStream(myOutput);
		} catch (FileNotFoundException e) {
			return "ERR";
		}
		System.setIn(in);
		System.setOut(out);

		var future = executor.submit(new Callable<Long>() {
			public Long call() throws Exception {
				long start = System.currentTimeMillis();
				{{ CLASS_NAME }}.main(new String[] {});
				long end = System.currentTimeMillis();
				return start - end;
			}
		});

		Long result = 0l;
		try {
			result = future.get(timeout, TimeUnit.MILLISECONDS);
		} catch (InterruptedException e) {
			return "ERR";
		} catch (ExecutionException e) {
			return "RTE";
		} catch (TimeoutException e) {
			return "TLE";
		} finally {
			try {
				in.close();
				out.close();
			} catch (IOException e) {
				return "ERR";
			}
		}

		if (result > timeout) {
			return "TLE";
		}

		try {
			var inStr = Files.readString(output.toPath());
			var outStr = Files.readString(myOutput.toPath());

			if (inStr.strip().equals(outStr.strip())) {
				return "AC";
			}
		} catch (IOException e) {
			return "ERR";
		}

		return "WA";
	}

	public static void main(String[] args) {
		File f = new File(testCaseDir);
		if (!f.isDirectory()) {
			System.err.println("Directory not found");
			System.exit(1);
			return;
		}

		var inputDir = Paths.get(testCaseDir, "in").toFile();
		var outputDir = Paths.get(testCaseDir, "out").toFile();
		var myOutputDir = Paths.get(testCaseDir, "myoutput").toFile();

		try {
			Files.createDirectories(myOutputDir.toPath());
		} catch (IOException e) {
			System.err.println("An error has occured.");
			System.exit(2);
			return;
		}

		if (!inputDir.isDirectory() || !outputDir.isDirectory()) {
			System.err.println("In/out directory not found");
			System.exit(1);
			return;
		}

		File[] inputs = inputDir.listFiles();
		int totalTestCases = inputs.length;
		if (totalTestCases == 0) {
			System.err.println("No test cases found");
			System.exit(1);
			return;
		}

		Arrays.sort(inputs, Comparator.comparingInt(o -> Integer.parseInt(getBaseName(o.getName()))));
		String[] results = new String[totalTestCases];

		for (int i = 0; i < totalTestCases; i++) {
			File input = inputs[i];
			String fname = input.toPath().getFileName().toString();
			File output = Paths.get(outputDir.toString(), fname).toFile();
			File myOutput = Paths.get(myOutputDir.toString(), fname).toFile();

			results[i] = runTest(input, output, myOutput);
			System.setOut(origOut);
			System.setIn(origIn);
		}

		for (String res : results) {
			System.out.print(res + ",");
		}
		executor.shutdown();
		return;
	}
}
