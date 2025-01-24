// <auto-generated />
//
// To parse this JSON data, add NuGet 'Newtonsoft.Json' then do:
//
//    using Dreamcaller.Schema;
//
//    var battleView = BattleView.FromJson(jsonString);

namespace Dreamcaller.Schema
{
    using System;
    using System.Collections.Generic;

    using System.Globalization;
    using Newtonsoft.Json;
    using Newtonsoft.Json.Converters;

    /// <summary>
    /// Represents the visual state of an ongoing dream battle
    /// </summary>
    public partial class BattleView
    {
        /// <summary>
        /// Visual state of cards in the game
        /// </summary>
        [JsonProperty("cards", Required = Required.Always)]
        public List<CardView> Cards { get; set; }

        /// <summary>
        /// User interaction options
        /// </summary>
        [JsonProperty("controls", Required = Required.Always)]
        public List<ControlView> Controls { get; set; }

        /// <summary>
        /// Opponent of user
        /// </summary>
        [JsonProperty("enemy", Required = Required.Always)]
        public PlayerView Enemy { get; set; }

        /// <summary>
        /// Unique identifier for this dream battle
        /// </summary>
        [JsonProperty("id", Required = Required.Always)]
        public string Id { get; set; }

        /// <summary>
        /// Describes the status of the game, e.g. which phase & step the game is in
        /// </summary>
        [JsonProperty("statusDescription", Required = Required.Always)]
        public string StatusDescription { get; set; }

        /// <summary>
        /// Player who is operating the client
        /// </summary>
        [JsonProperty("user", Required = Required.Always)]
        public PlayerView User { get; set; }
    }

    /// <summary>
    /// Represents the visual state of a card or ability in a game
    /// </summary>
    public partial class CardView
    {
        /// <summary>
        /// Card back image
        /// </summary>
        [JsonProperty("cardBack", Required = Required.Always)]
        public string CardBack { get; set; }

        /// <summary>
        /// Face up/face down state for this card
        /// </summary>
        [JsonProperty("cardFacing", Required = Required.Always)]
        public CardFacing CardFacing { get; set; }

        /// <summary>
        /// Optionally, a position at which to create this card.
        ///
        /// If this card does not already exist, it will be created at this position before being
        /// animated to [Self::position].
        /// </summary>
        [JsonProperty("createPosition")]
        public ObjectPosition CreatePosition { get; set; }

        /// <summary>
        /// Optionally, a position at which to destroy this card.
        ///
        /// If provided, the card will be animated to this position before being destroyed.
        /// </summary>
        [JsonProperty("destroyPosition")]
        public ObjectPosition DestroyPosition { get; set; }

        /// <summary>
        /// Identifier for this card
        /// </summary>
        [JsonProperty("id", Required = Required.Always)]
        public ClientCardId Id { get; set; }

        /// <summary>
        /// Position of this card in the UI
        /// </summary>
        [JsonProperty("position", Required = Required.Always)]
        public ObjectPosition Position { get; set; }

        /// <summary>
        /// If this card is revealed to the viewer, contains information on the revealed face of the
        /// card.
        /// </summary>
        [JsonProperty("revealed")]
        public RevealedCardView Revealed { get; set; }

        /// <summary>
        /// True if this card is in a hidden zone but known to one or more opponents
        /// </summary>
        [JsonProperty("revealedToOpponents", Required = Required.Always)]
        public bool RevealedToOpponents { get; set; }
    }

    /// <summary>
    /// Represents the position of some object in the UI
    ///
    /// Position of this card in the UI
    /// </summary>
    public partial class ObjectPosition
    {
        /// <summary>
        /// Position category
        /// </summary>
        [JsonProperty("position", Required = Required.Always)]
        public Position Position { get; set; }

        /// <summary>
        /// Sorting key, determines order within the position
        /// </summary>
        [JsonProperty("sortingKey", Required = Required.Always)]
        public long SortingKey { get; set; }

        /// <summary>
        /// Sub-key, used to break ties in sorting
        /// </summary>
        [JsonProperty("sortingSubKey", Required = Required.Always)]
        public long SortingSubKey { get; set; }
    }

    /// <summary>
    /// Object is in a player's hand
    ///
    /// Object is on top of a player's deck
    ///
    /// Object is shuffled into a player's deck
    ///
    /// Object is in a player's void
    ///
    /// Object is in this player's banished zone
    ///
    /// Object is on the battlefield
    /// </summary>
    public partial class PositionClass
    {
        [JsonProperty("inHand", Required = Required.DisallowNull, NullValueHandling = NullValueHandling.Ignore)]
        public DisplayPlayer? InHand { get; set; }

        [JsonProperty("onTopOfDeck", Required = Required.DisallowNull, NullValueHandling = NullValueHandling.Ignore)]
        public DisplayPlayer? OnTopOfDeck { get; set; }

        [JsonProperty("inDeck", Required = Required.DisallowNull, NullValueHandling = NullValueHandling.Ignore)]
        public DisplayPlayer? InDeck { get; set; }

        [JsonProperty("inVoid", Required = Required.DisallowNull, NullValueHandling = NullValueHandling.Ignore)]
        public DisplayPlayer? InVoid { get; set; }

        [JsonProperty("inBanished", Required = Required.DisallowNull, NullValueHandling = NullValueHandling.Ignore)]
        public DisplayPlayer? InBanished { get; set; }

        [JsonProperty("onBattlefield", Required = Required.DisallowNull, NullValueHandling = NullValueHandling.Ignore)]
        public DisplayPlayer? OnBattlefield { get; set; }
    }

    /// <summary>
    /// Identifier for this card
    ///
    /// Identifies a card in client code
    ///
    /// Client-opaque serialized value.
    /// </summary>
    public partial class ClientCardId
    {
        [JsonProperty("cardId", Required = Required.DisallowNull, NullValueHandling = NullValueHandling.Ignore)]
        public string CardId { get; set; }

        [JsonProperty("activatedAbilityId", Required = Required.DisallowNull, NullValueHandling = NullValueHandling.Ignore)]
        public string ActivatedAbilityId { get; set; }

        [JsonProperty("triggeredAbilityId", Required = Required.DisallowNull, NullValueHandling = NullValueHandling.Ignore)]
        public string TriggeredAbilityId { get; set; }
    }

    /// <summary>
    /// Visual state of a revealed card
    /// </summary>
    public partial class RevealedCardView
    {
        /// <summary>
        /// True if this card can be dragged by the player.
        ///
        /// The set of valid drag targets is set on the GameView. All draggable cards can be dragged
        /// to and reordered within any valid target.
        /// </summary>
        [JsonProperty("canDrag", Required = Required.Always)]
        public bool CanDrag { get; set; }

        /// <summary>
        /// Type or subtype of this card
        /// </summary>
        [JsonProperty("cardType", Required = Required.Always)]
        public string CardType { get; set; }

        /// <summary>
        /// Cost of this card
        /// </summary>
        [JsonProperty("cost", Required = Required.Always)]
        public long Cost { get; set; }

        /// <summary>
        /// Frame to display for this card
        /// </summary>
        [JsonProperty("frame", Required = Required.Always)]
        public CardFrame Frame { get; set; }

        /// <summary>
        /// Image URL for this card
        /// </summary>
        [JsonProperty("image", Required = Required.Always)]
        public DisplayImage Image { get; set; }

        /// <summary>
        /// True if this card can be played during the opponent's turn
        /// </summary>
        [JsonProperty("isFast", Required = Required.Always)]
        public bool IsFast { get; set; }

        /// <summary>
        /// Name of this card
        /// </summary>
        [JsonProperty("name", Required = Required.Always)]
        public string Name { get; set; }

        /// <summary>
        /// Rules text to display for this face
        /// </summary>
        [JsonProperty("rulesText", Required = Required.Always)]
        public string RulesText { get; set; }

        /// <summary>
        /// Spark value for this card
        /// </summary>
        [JsonProperty("spark")]
        public long? Spark { get; set; }

        /// <summary>
        /// Visual status of this card
        /// </summary>
        [JsonProperty("status")]
        public RevealedCardStatus? Status { get; set; }
    }

    /// <summary>
    /// Image URL for this card
    /// </summary>
    public partial class DisplayImage
    {
        /// <summary>
        /// Image URL for this card
        /// </summary>
        [JsonProperty("image", Required = Required.Always)]
        public string Image { get; set; }

        /// <summary>
        /// X offset position of this image
        /// </summary>
        [JsonProperty("imageOffsetX")]
        public long? ImageOffsetX { get; set; }

        /// <summary>
        /// Y offset position of this image
        /// </summary>
        [JsonProperty("imageOffsetY")]
        public long? ImageOffsetY { get; set; }
    }

    /// <summary>
    /// User interaction options
    /// </summary>
    public partial class ControlView
    {
        [JsonProperty("button", Required = Required.Always)]
        public ButtonView Button { get; set; }
    }

    /// <summary>
    /// Button to perform some game action
    /// </summary>
    public partial class ButtonView
    {
        [JsonProperty("kind", Required = Required.Always)]
        public ButtonKind Kind { get; set; }

        [JsonProperty("label", Required = Required.Always)]
        public string Label { get; set; }
    }

    /// <summary>
    /// Opponent of user
    ///
    /// Represents the visual state of a player in a game
    ///
    /// Player who is operating the client
    /// </summary>
    public partial class PlayerView
    {
        /// <summary>
        /// Can this player currently take a game action?
        /// </summary>
        [JsonProperty("canAct", Required = Required.Always)]
        public bool CanAct { get; set; }

        /// <summary>
        /// Current score total
        /// </summary>
        [JsonProperty("score", Required = Required.Always)]
        public long Score { get; set; }
    }

    /// <summary>
    /// Face up/face down state for this card
    ///
    /// Whether a card is face-down or face-up
    /// </summary>
    public enum CardFacing { FaceDown, FaceUp };

    /// <summary>
    /// Object position used in interface elements like the deck viewer which don't rely on game
    /// positioning.
    ///
    /// Object is not visible.
    ///
    /// Object is prominently revealed, being shown at a large size after being played.
    ///
    /// Object is on the stack
    ///
    /// Object is being displayed in a card browser, e.g. to select from a list of cards while
    /// searching
    ///
    /// Object is being displayed in a list of cards available to select in a card selector.
    ///
    /// Object has just been revealed to this viewer
    ///
    /// Object is in a temporary holding space for cards in hand while resolving some other 'play
    /// card' ability.
    /// </summary>
    public enum PositionEnum { Browser, CardSelectionChoices, Default, HandStorage, Offscreen, OnStack, Played, Revealed };

    /// <summary>
    /// Identifies a player in the context of the user interface.
    ///
    /// Player who is currently operating the client
    ///
    /// Opponent of user, i.e. the AI enemy
    /// </summary>
    public enum DisplayPlayer { Enemy, User };

    /// <summary>
    /// Frame to display for this card
    /// </summary>
    public enum CardFrame { Character, Event };

    public enum RevealedCardStatus { CanPlay, CanSelect, Selected };

    /// <summary>
    /// Controls color for buttons
    ///
    /// Emphasized button, primary game action
    ///
    /// Deemphasized button, additional game actions
    /// </summary>
    public enum ButtonKind { Default, Primary };

    /// <summary>
    /// Position category
    ///
    /// Possible types of display positions
    /// </summary>
    public partial struct Position
    {
        public PositionEnum? Enum;
        public PositionClass PositionClass;

        public static implicit operator Position(PositionEnum Enum) => new Position { Enum = Enum };
        public static implicit operator Position(PositionClass PositionClass) => new Position { PositionClass = PositionClass };
    }

    public partial class BattleView
    {
        public static BattleView FromJson(string json) => JsonConvert.DeserializeObject<BattleView>(json, Dreamcaller.Schema.Converter.Settings);
    }

    public static class Serialize
    {
        public static string ToJson(this BattleView self) => JsonConvert.SerializeObject(self, Dreamcaller.Schema.Converter.Settings);
    }

    internal static class Converter
    {
        public static readonly JsonSerializerSettings Settings = new JsonSerializerSettings
        {
            MetadataPropertyHandling = MetadataPropertyHandling.Ignore,
            DateParseHandling = DateParseHandling.None,
            Converters =
            {
                CardFacingConverter.Singleton,
                PositionConverter.Singleton,
                DisplayPlayerConverter.Singleton,
                PositionEnumConverter.Singleton,
                CardFrameConverter.Singleton,
                RevealedCardStatusConverter.Singleton,
                ButtonKindConverter.Singleton,
                new IsoDateTimeConverter { DateTimeStyles = DateTimeStyles.AssumeUniversal }
            },
        };
    }

    internal class CardFacingConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(CardFacing) || t == typeof(CardFacing?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "faceDown":
                    return CardFacing.FaceDown;
                case "faceUp":
                    return CardFacing.FaceUp;
            }
            throw new Exception("Cannot unmarshal type CardFacing");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (CardFacing)untypedValue;
            switch (value)
            {
                case CardFacing.FaceDown:
                    serializer.Serialize(writer, "faceDown");
                    return;
                case CardFacing.FaceUp:
                    serializer.Serialize(writer, "faceUp");
                    return;
            }
            throw new Exception("Cannot marshal type CardFacing");
        }

        public static readonly CardFacingConverter Singleton = new CardFacingConverter();
    }

    internal class PositionConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(Position) || t == typeof(Position?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            switch (reader.TokenType)
            {
                case JsonToken.String:
                case JsonToken.Date:
                    var stringValue = serializer.Deserialize<string>(reader);
                    switch (stringValue)
                    {
                        case "browser":
                            return new Position { Enum = PositionEnum.Browser };
                        case "cardSelectionChoices":
                            return new Position { Enum = PositionEnum.CardSelectionChoices };
                        case "default":
                            return new Position { Enum = PositionEnum.Default };
                        case "handStorage":
                            return new Position { Enum = PositionEnum.HandStorage };
                        case "offscreen":
                            return new Position { Enum = PositionEnum.Offscreen };
                        case "onStack":
                            return new Position { Enum = PositionEnum.OnStack };
                        case "played":
                            return new Position { Enum = PositionEnum.Played };
                        case "revealed":
                            return new Position { Enum = PositionEnum.Revealed };
                    }
                    break;
                case JsonToken.StartObject:
                    var objectValue = serializer.Deserialize<PositionClass>(reader);
                    return new Position { PositionClass = objectValue };
            }
            throw new Exception("Cannot unmarshal type Position");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            var value = (Position)untypedValue;
            if (value.Enum != null)
            {
                switch (value.Enum)
                {
                    case PositionEnum.Browser:
                        serializer.Serialize(writer, "browser");
                        return;
                    case PositionEnum.CardSelectionChoices:
                        serializer.Serialize(writer, "cardSelectionChoices");
                        return;
                    case PositionEnum.Default:
                        serializer.Serialize(writer, "default");
                        return;
                    case PositionEnum.HandStorage:
                        serializer.Serialize(writer, "handStorage");
                        return;
                    case PositionEnum.Offscreen:
                        serializer.Serialize(writer, "offscreen");
                        return;
                    case PositionEnum.OnStack:
                        serializer.Serialize(writer, "onStack");
                        return;
                    case PositionEnum.Played:
                        serializer.Serialize(writer, "played");
                        return;
                    case PositionEnum.Revealed:
                        serializer.Serialize(writer, "revealed");
                        return;
                }
            }
            if (value.PositionClass != null)
            {
                serializer.Serialize(writer, value.PositionClass);
                return;
            }
            throw new Exception("Cannot marshal type Position");
        }

        public static readonly PositionConverter Singleton = new PositionConverter();
    }

    internal class DisplayPlayerConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(DisplayPlayer) || t == typeof(DisplayPlayer?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "enemy":
                    return DisplayPlayer.Enemy;
                case "user":
                    return DisplayPlayer.User;
            }
            throw new Exception("Cannot unmarshal type DisplayPlayer");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (DisplayPlayer)untypedValue;
            switch (value)
            {
                case DisplayPlayer.Enemy:
                    serializer.Serialize(writer, "enemy");
                    return;
                case DisplayPlayer.User:
                    serializer.Serialize(writer, "user");
                    return;
            }
            throw new Exception("Cannot marshal type DisplayPlayer");
        }

        public static readonly DisplayPlayerConverter Singleton = new DisplayPlayerConverter();
    }

    internal class PositionEnumConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(PositionEnum) || t == typeof(PositionEnum?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "browser":
                    return PositionEnum.Browser;
                case "cardSelectionChoices":
                    return PositionEnum.CardSelectionChoices;
                case "default":
                    return PositionEnum.Default;
                case "handStorage":
                    return PositionEnum.HandStorage;
                case "offscreen":
                    return PositionEnum.Offscreen;
                case "onStack":
                    return PositionEnum.OnStack;
                case "played":
                    return PositionEnum.Played;
                case "revealed":
                    return PositionEnum.Revealed;
            }
            throw new Exception("Cannot unmarshal type PositionEnum");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (PositionEnum)untypedValue;
            switch (value)
            {
                case PositionEnum.Browser:
                    serializer.Serialize(writer, "browser");
                    return;
                case PositionEnum.CardSelectionChoices:
                    serializer.Serialize(writer, "cardSelectionChoices");
                    return;
                case PositionEnum.Default:
                    serializer.Serialize(writer, "default");
                    return;
                case PositionEnum.HandStorage:
                    serializer.Serialize(writer, "handStorage");
                    return;
                case PositionEnum.Offscreen:
                    serializer.Serialize(writer, "offscreen");
                    return;
                case PositionEnum.OnStack:
                    serializer.Serialize(writer, "onStack");
                    return;
                case PositionEnum.Played:
                    serializer.Serialize(writer, "played");
                    return;
                case PositionEnum.Revealed:
                    serializer.Serialize(writer, "revealed");
                    return;
            }
            throw new Exception("Cannot marshal type PositionEnum");
        }

        public static readonly PositionEnumConverter Singleton = new PositionEnumConverter();
    }

    internal class CardFrameConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(CardFrame) || t == typeof(CardFrame?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "character":
                    return CardFrame.Character;
                case "event":
                    return CardFrame.Event;
            }
            throw new Exception("Cannot unmarshal type CardFrame");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (CardFrame)untypedValue;
            switch (value)
            {
                case CardFrame.Character:
                    serializer.Serialize(writer, "character");
                    return;
                case CardFrame.Event:
                    serializer.Serialize(writer, "event");
                    return;
            }
            throw new Exception("Cannot marshal type CardFrame");
        }

        public static readonly CardFrameConverter Singleton = new CardFrameConverter();
    }

    internal class RevealedCardStatusConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(RevealedCardStatus) || t == typeof(RevealedCardStatus?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "canPlay":
                    return RevealedCardStatus.CanPlay;
                case "canSelect":
                    return RevealedCardStatus.CanSelect;
                case "selected":
                    return RevealedCardStatus.Selected;
            }
            throw new Exception("Cannot unmarshal type RevealedCardStatus");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (RevealedCardStatus)untypedValue;
            switch (value)
            {
                case RevealedCardStatus.CanPlay:
                    serializer.Serialize(writer, "canPlay");
                    return;
                case RevealedCardStatus.CanSelect:
                    serializer.Serialize(writer, "canSelect");
                    return;
                case RevealedCardStatus.Selected:
                    serializer.Serialize(writer, "selected");
                    return;
            }
            throw new Exception("Cannot marshal type RevealedCardStatus");
        }

        public static readonly RevealedCardStatusConverter Singleton = new RevealedCardStatusConverter();
    }

    internal class ButtonKindConverter : JsonConverter
    {
        public override bool CanConvert(Type t) => t == typeof(ButtonKind) || t == typeof(ButtonKind?);

        public override object ReadJson(JsonReader reader, Type t, object existingValue, JsonSerializer serializer)
        {
            if (reader.TokenType == JsonToken.Null) return null;
            var value = serializer.Deserialize<string>(reader);
            switch (value)
            {
                case "default":
                    return ButtonKind.Default;
                case "primary":
                    return ButtonKind.Primary;
            }
            throw new Exception("Cannot unmarshal type ButtonKind");
        }

        public override void WriteJson(JsonWriter writer, object untypedValue, JsonSerializer serializer)
        {
            if (untypedValue == null)
            {
                serializer.Serialize(writer, null);
                return;
            }
            var value = (ButtonKind)untypedValue;
            switch (value)
            {
                case ButtonKind.Default:
                    serializer.Serialize(writer, "default");
                    return;
                case ButtonKind.Primary:
                    serializer.Serialize(writer, "primary");
                    return;
            }
            throw new Exception("Cannot marshal type ButtonKind");
        }

        public static readonly ButtonKindConverter Singleton = new ButtonKindConverter();
    }
}
