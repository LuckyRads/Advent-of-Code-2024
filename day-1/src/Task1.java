import java.io.BufferedReader;
import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

class Task1 {

    public static void main(String[] args) {
        File inputFile = new File("./input/input.txt");

        List<Integer> locationIds1 = new ArrayList<>();
        List<Integer> locationIds2 = new ArrayList<>();

        try (BufferedReader bufferedReader = new BufferedReader(
                new InputStreamReader(inputFile.toURI().toURL().openStream()))) {
            String line;
            while ((line = bufferedReader.readLine()) != null) {
                String[] locationIdArray = line.split("\\s+");
                locationIds1.add(Integer.parseInt(locationIdArray[0]));
                locationIds2.add(Integer.parseInt(locationIdArray[1]));
            }
        } catch (IOException e) {
            e.printStackTrace();
        }

        locationIds1.sort(Comparator.comparingInt(Integer::intValue));
        locationIds2.sort(Comparator.comparingInt(Integer::intValue));

        int diffSum = 0;

        for (int i = 0; i < locationIds1.size(); i++) {
            diffSum += Math.abs(locationIds1.get(i) - locationIds2.get(i));
        }

        try (FileWriter fileWriter = new FileWriter(new File("./output/task1.txt"))) {
            fileWriter.write(diffSum + "\n");
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

}