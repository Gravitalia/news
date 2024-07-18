import tensorflow as tf
import tensorflow_hub as tfhub
import tensorflow_text as text
from official.nlp import optimization

import matplotlib.pyplot as plt

tf.get_logger().setLevel("ERROR")

"""# Create dataset."""

AUTOTUNE = tf.data.AUTOTUNE
batch_size = 64
seed = 42

raw_train_ds = tf.keras.utils.text_dataset_from_directory(
    "drive/MyDrive/datasets",
    batch_size=batch_size,
    validation_split=0.2,
    subset="training",
    seed=seed)

class_names = raw_train_ds.class_names
train_ds = raw_train_ds.cache().prefetch(buffer_size=AUTOTUNE)

val_ds = tf.keras.utils.text_dataset_from_directory(
    "drive/MyDrive/datasets",
    batch_size=batch_size,
    validation_split=0.2,
    subset="validation",
    seed=seed)

val_ds = val_ds.cache().prefetch(buffer_size=AUTOTUNE)

test_ds = tf.keras.utils.text_dataset_from_directory(
    "drive/MyDrive/datasets",
    batch_size=batch_size)

test_ds = test_ds.cache().prefetch(buffer_size=AUTOTUNE)

"""# Load BERT models."""

tfhub_model = "https://tfhub.dev/tensorflow/bert_multi_cased_L-12_H-768_A-12/3"
thhub_preprocess = "https://tfhub.dev/tensorflow/bert_multi_cased_preprocess/3"

bert_model = tfhub.KerasLayer(tfhub_model)

bert_preprocess_model = tfhub.KerasLayer(thhub_preprocess)
