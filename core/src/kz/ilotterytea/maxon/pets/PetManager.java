package kz.ilotterytea.maxon.pets;

import com.badlogic.gdx.assets.AssetManager;
import com.badlogic.gdx.utils.JsonReader;
import com.badlogic.gdx.utils.JsonValue;
import kz.ilotterytea.maxon.assets.loaders.Text;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashSet;
import java.util.Optional;

public class PetManager {
    private final HashSet<Pet> pets;
    private final AssetManager assetManager;
    private final Logger logger = LoggerFactory.getLogger(PetManager.class);

    public PetManager(final AssetManager assetManager) {
        this.pets = new HashSet<>();
        this.assetManager = assetManager;
    }

    public void load() {
        pets.clear();

        String data = assetManager.get("data/pets.json", Text.class).getString();
        JsonValue root = new JsonReader().parse(data);

        for (JsonValue child : root.iterator()) {
            String id = child.getString("id");
            double price = child.getDouble("price");
            double multiplier = child.getDouble("multiplier");

            JsonValue iconData = child.get("icon_data");
            int iconColumns = iconData.getInt("columns");
            int iconRows = iconData.getInt("rows");

            Pet pet = Pet.create(id, price, multiplier, iconColumns, iconRows);
            pets.add(pet);
        }

        logger.info("Loaded {} pets", pets.size());
    }

    public Optional<Pet> getPet(String id) {
        return pets.stream().filter(x -> x.getId().equals(id)).findFirst();
    }

    public HashSet<Pet> getPets() {
        return pets;
    }
}