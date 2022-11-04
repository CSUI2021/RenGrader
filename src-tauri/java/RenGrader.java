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
	private static String inputFilePath = "{{ INPUT_PATH }}";
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
		}

		if (result > timeout) {
			return "TLE";
		}

		try {
			var inStr = Files.readString(output.toPath()).replace("\r\n", "\n");
			var outStr = Files.readString(myOutput.toPath()).replace("\r\n", "\n");

			if (inStr.strip().equals(outStr.strip())) {
				return "AC";
			}
		} catch (IOException e) {
			return "ERR";
		}

		return "WA";
	}

	public static void main(String[] args) {
		var inputDir = Paths.get(testCaseDir, "in").toFile();
		var outputDir = Paths.get(testCaseDir, "out").toFile();
		var myOutputDir = Paths.get(testCaseDir, "myoutput").toFile();
		
		File input = new File(inputFilePath);
		String fname = input.toPath().getFileName().toString();
		File output = Paths.get(outputDir.toString(), fname).toFile();
		File myOutput = Paths.get(myOutputDir.toString(), fname).toFile();

		try {
			Files.createDirectories(myOutputDir.toPath());
		} catch (IOException e) {
			System.err.println("An error has occured.");
			System.exit(2);
			return;
		}

		var res = "";
		InputStream in;
		PrintStream out;
		try {
			in = new FileInputStream(input);
			out = new PrintStream(myOutput);
		} catch (FileNotFoundException e) {
			System.err.println("Input/output file not found.");
			System.exit(2);
			return;
		}

		System.setIn(in);
		System.setOut(out);

		res = runTest(input, output, myOutput);
		try {
			in.close();
			out.close();
		} catch (IOException e) {
			res = "ERR";
		}

		System.setOut(origOut);
		System.setIn(origIn);

		System.out.print(res);
		executor.shutdown();
		return;
	}
}
