import java.io.BufferedReader;
import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

class Task2 {

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

        List<Integer> occurrenceCounts = locationIds1.stream()
                .map(locationId1 -> locationIds2.stream()
                        .filter(locationId2 -> Objects.equals(locationId1, locationId2)).count())
                .map(Long::intValue)
                .toList();

        int similarityScore = 0;
        for (int i = 0; i < locationIds1.size(); i++) {
            similarityScore += locationIds1.get(i) * occurrenceCounts.get(i);
        }

        try (FileWriter fileWriter = new FileWriter(new File("./output/task2.txt"))) {
            fileWriter.write(similarityScore + "\n");
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

}