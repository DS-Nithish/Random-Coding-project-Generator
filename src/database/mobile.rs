use crate::models::{Difficulty, Domain, ProjectIdea};

pub fn get_projects() -> Vec<ProjectIdea> {
    vec![
        // 1. Beginner Mobile 1
        ProjectIdea {
            id: "mob-beg-1".into(),
            title: "Hydration & Habit Water Tracker App".into(),
            domain: Domain::MobileApp,
            difficulty: Difficulty::Beginner,
            description: "A clean mobile app that tracks daily water intake goals, logs glass increments with fluid wave animations, and sends reminder alerts.".into(),
            requirements: vec![
                "Daily water consumption goal setting and progress percentage ring".into(),
                "Quick-tap log buttons (+250ml, +500ml) with undo capability".into(),
                "Local device storage persistence for 7-day intake history".into(),
                "Custom notification reminders for hydration intervals".into(),
            ],
            technologies: vec!["React Native / Flutter".into(), "Local Storage / SQLite".into(), "Mobile Push Notifications".into()],
            duration: "8 - 12 hours".into(),
            steps: vec![
                "Step 1: Scaffold mobile screen layout with target bottle visual.".into(),
                "Step 2: Connect tap gestures to state increment and wave fill level.".into(),
                "Step 3: Persist daily totals by date key in AsyncStorage / SharedPreferences.".into(),
                "Step 4: Configure local notification scheduler for hourly alerts.".into(),
            ],
            starter_code_language: "javascript".into(),
            starter_code_filename: "WaterTracker.jsx".into(),
            starter_code: r#"import React, { useState, useEffect } from 'react';
import { View, Text, TouchableOpacity, StyleSheet } from 'react-native';

export default function WaterTracker() {
  const [currentMl, setCurrentMl] = useState(0);
  const targetMl = 2500;

  const addWater = (amount) => {
    setCurrentMl(prev => Math.min(targetMl, prev + amount));
  };

  const progress = Math.round((currentMl / targetMl) * 100);

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Daily Hydration</Text>
      <View style={styles.circle}>
        <Text style={styles.percent}>{progress}%</Text>
        <Text style={styles.subtitle}>{currentMl} / {targetMl} ml</Text>
      </View>
      <View style={styles.buttonRow}>
        <TouchableOpacity style={styles.button} onPress={() => addWater(250)}>
          <Text style={styles.btnText}>+250ml</Text>
        </TouchableOpacity>
        <TouchableOpacity style={styles.button} onPress={() => addWater(500)}>
          <Text style={styles.btnText}>+500ml</Text>
        </TouchableOpacity>
        <TouchableOpacity style={[styles.button, styles.resetBtn]} onPress={() => setCurrentMl(0)}>
          <Text style={styles.btnText}>Reset</Text>
        </TouchableOpacity>
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: '#0f172a', alignItems: 'center', justifyContent: 'center' },
  title: { fontSize: 24, fontWeight: 'bold', color: '#fff', marginBottom: 20 },
  circle: { width: 200, height: 200, borderRadius: 100, borderWidth: 8, borderColor: '#38bdf8', alignItems: 'center', justifyContent: 'center', marginBottom: 40 },
  percent: { fontSize: 44, fontWeight: 'bold', color: '#38bdf8' },
  subtitle: { fontSize: 16, color: '#94a3b8' },
  buttonRow: { flexDirection: 'row', gap: 12 },
  button: { backgroundColor: '#0284c7', paddingVertical: 12, paddingHorizontal: 18, borderRadius: 12 },
  resetBtn: { backgroundColor: '#ef4444' },
  btnText: { color: '#fff', fontWeight: 'bold' }
});
"#.into(),
            documentation: r#"# Hydration & Habit Water Tracker

Mobile tracker supporting daily goals, quick logging, and reset gestures.
"#.into(),
        },

        // 2. Beginner Mobile 2
        ProjectIdea {
            id: "mob-beg-2".into(),
            title: "Micro-Podcast & Voice Memo Recorder".into(),
            domain: Domain::MobileApp,
            difficulty: Difficulty::Beginner,
            description: "Voice memo and short podcast audio recorder with live audio wave level visualizer, playback scrubber, and audio file tagging.".into(),
            requirements: vec![
                "Microphone recording permission handling and high-quality AAC/M4A capture".into(),
                "Visual audio amplitude level meter during live recording".into(),
                "Play, pause, skip, and scrub timeline controls".into(),
                "Rename, tag, and export audio recordings".into(),
            ],
            technologies: vec!["React Native Audio / Flutter Sound".into(), "Device File System".into(), "Permissions API".into()],
            duration: "8 - 14 hours".into(),
            steps: vec![
                "Step 1: Set up iOS and Android microphone recording permissions in manifests.".into(),
                "Step 2: Connect audio recording engine and buffer amplitude polling.".into(),
                "Step 3: Save recordings to local app document storage with unique timestamps.".into(),
                "Step 4: Build audio player list item with progress scrubber.".into(),
            ],
            starter_code_language: "javascript".into(),
            starter_code_filename: "VoiceRecorder.jsx".into(),
            starter_code: r#"// Voice Recorder Component Template
import React, { useState } from 'react';
import { View, Text, Button, Alert } from 'react-native';

export default function VoiceRecorder() {
  const [isRecording, setIsRecording] = useState(false);
  const [recordings, setRecordings] = useState([]);

  const toggleRecord = () => {
    if (isRecording) {
      setIsRecording(false);
      setRecordings(prev => [...prev, { id: Date.now(), name: `Memo #${prev.length + 1}` }]);
    } else {
      setIsRecording(true);
    }
  };

  return (
    <View style={{ flex: 1, padding: 24, justifyContent: 'center' }}>
      <Text style={{ fontSize: 20, textAlign: 'center', marginBottom: 20 }}>
        {isRecording ? '🔴 Recording...' : 'Tap to Record'}
      </Text>
      <Button title={isRecording ? 'Stop' : 'Start Recording'} onPress={toggleRecord} />
    </View>
  );
}
"#.into(),
            documentation: r#"# Micro-Podcast Voice Recorder

Clean mobile recording workflow with audio device permissions.
"#.into(),
        },

        // 3. Beginner Mobile 3
        ProjectIdea {
            id: "mob-beg-3".into(),
            title: "Offline Grocery & Pantry Inventory".into(),
            domain: Domain::MobileApp,
            difficulty: Difficulty::Beginner,
            description: "Smart grocery list and pantry stock organizer with barcode scanning, expiration date warnings, and recipe ingredient matching.".into(),
            requirements: vec![
                "Add, check off, and swipe-to-delete grocery items".into(),
                "Camera barcode scanning to quickly lookup product titles".into(),
                "Pantry shelf expiration date color badges (red = expires soon)".into(),
                "Share shopping list via system share sheet (SMS / WhatsApp)".into(),
            ],
            technologies: vec!["React Native / Flutter".into(), "Camera / Barcode Scanner".into(), "AsyncStorage".into()],
            duration: "7 - 11 hours".into(),
            steps: vec![
                "Step 1: Scaffold categorized list view (Produce, Dairy, Pantry, Frozen).".into(),
                "Step 2: Integrate camera barcode detection library.".into(),
                "Step 3: Implement item swipe actions using Reanimated or Dismissible.".into(),
                "Step 4: Add system Share sheet integration for sending text lists.".into(),
            ],
            starter_code_language: "javascript".into(),
            starter_code_filename: "PantryList.jsx".into(),
            starter_code: r#"import React, { useState } from 'react';
import { View, Text, FlatList, TextInput, TouchableOpacity } from 'react-native';

export default function PantryList() {
  const [items, setItems] = useState([
    { id: '1', name: 'Almond Milk', daysLeft: 2 },
    { id: '2', name: 'Avocados', daysLeft: 4 }
  ]);
  const [text, setText] = useState('');

  const addItem = () => {
    if (!text.trim()) return;
    setItems([...items, { id: Date.now().toString(), name: text, daysLeft: 7 }]);
    setText('');
  };

  return (
    <View style={{ flex: 1, padding: 20, backgroundColor: '#fff' }}>
      <TextInput placeholder="Add grocery item..." value={text} onChangeText={setText} onSubmitEditing={addItem} style={{ borderWidth: 1, padding: 10, borderRadius: 8, marginBottom: 15 }} />
      <FlatList
        data={items}
        keyExtractor={item => item.id}
        renderItem={({ item }) => (
          <View style={{ padding: 14, backgroundColor: '#f1f5f9', borderRadius: 8, marginBottom: 8, flexDirection: 'row', justifyContent: 'space-between' }}>
            <Text style={{ fontSize: 16 }}>{item.name}</Text>
            <Text style={{ color: item.daysLeft <= 2 ? '#ef4444' : '#10b981', fontWeight: 'bold' }}>{item.daysLeft} days left</Text>
          </View>
        )}
      />
    </View>
  );
}
"#.into(),
            documentation: r#"# Offline Grocery & Pantry Inventory

Track supplies and expiration dates seamlessly on device.
"#.into(),
        },

        // 4. Intermediate Mobile 1
        ProjectIdea {
            id: "mob-int-1".into(),
            title: "GPS Trail Runner & Route Mapper".into(),
            domain: Domain::MobileApp,
            difficulty: Difficulty::Intermediate,
            description: "Fitness run tracker that uses background geolocation to draw GPS paths on an interactive map, calculating pace, elevation gain, and burned calories.".into(),
            requirements: vec![
                "Background GPS tracking service with battery-efficient location throttling".into(),
                "Interactive Mapbox or Google Maps polyline overlay of user trail".into(),
                "Real-time HUD displaying speed, split mile pace, distance, and duration".into(),
                "GPX file export for sharing runs to Strava or Garmin".into(),
            ],
            technologies: vec!["React Native Maps / Flutter Mapbox".into(), "Background Location Service".into(), "Haversine Distance Algorithm".into(), "SQLite".into()],
            duration: "20 - 32 hours".into(),
            steps: vec![
                "Step 1: Set up background location permissions (iOS Always, Android ACCESS_BACKGROUND_LOCATION).".into(),
                "Step 2: Collect geolocation coordinates and filter out GPS jitter using Kalman filter.".into(),
                "Step 3: Render real-time polyline trajectory on the vector map.".into(),
                "Step 4: Calculate rolling pace splits and elevation changes.".into(),
                "Step 5: Serialize trail waypoints to standard XML-based GPX schema.".into(),
            ],
            starter_code_language: "javascript".into(),
            starter_code_filename: "RunTracker.jsx".into(),
            starter_code: r#"// GPS Distance calculation using Haversine formula
function haversineDistance(c1, c2) {
  const R = 6371e3; // Earth radius in meters
  const toRad = deg => (deg * Math.PI) / 180;
  const dLat = toRad(c2.latitude - c1.latitude);
  const dLon = toRad(c2.longitude - c1.longitude);
  const a = Math.sin(dLat / 2) ** 2 +
            Math.cos(toRad(c1.latitude)) * Math.cos(toRad(c2.latitude)) *
            Math.sin(dLon / 2) ** 2;
  return R * 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
}

export function calculateRunStats(coordinates, elapsedSeconds) {
  let totalDistanceMeters = 0;
  for (let i = 1; i < coordinates.length; i++) {
    totalDistanceMeters += haversineDistance(coordinates[i - 1], coordinates[i]);
  }
  const km = totalDistanceMeters / 1000;
  const paceSecondsPerKm = km > 0 ? elapsedSeconds / km : 0;
  return { distanceKm: km.toFixed(2), paceSecPerKm: Math.round(paceSecondsPerKm) };
}
"#.into(),
            documentation: r#"# GPS Trail Runner & Route Mapper

Robust background geolocation handling and accurate geodesic math.
"#.into(),
        },

        // 5. Intermediate Mobile 2
        ProjectIdea {
            id: "mob-int-2".into(),
            title: "Encrypted Vault & Password Keeper".into(),
            domain: Domain::MobileApp,
            difficulty: Difficulty::Intermediate,
            description: "Zero-knowledge mobile password manager with biometric authentication (FaceID/Fingerprint), AES-256 GCM vault encryption, and password generator.".into(),
            requirements: vec![
                "Biometric unlock (Face ID / Fingerprint / Keychain / Android Keystore)".into(),
                "Client-side AES-256 GCM encryption with PBKDF2 / Argon2 key derivation".into(),
                "Strong password generator with customizable length, symbols, and entropy score".into(),
                "Auto-clear clipboard after 30 seconds for copied credentials".into(),
            ],
            technologies: vec!["React Native Keychain / Flutter Biometrics".into(), "CryptoJS / Libsodium".into(), "Secure Storage".into()],
            duration: "18 - 28 hours".into(),
            steps: vec![
                "Step 1: Implement master password prompt with PBKDF2 100,000 salt iterations.".into(),
                "Step 2: Store derived vault master key in hardware Secure Enclave / Keystore.".into(),
                "Step 3: Encrypt/decrypt payload records upon vault lock/unlock.".into(),
                "Step 4: Integrate Biometric prompt for quick unlocking.".into(),
                "Step 5: Add background app state listener to auto-lock vault when backgrounded.".into(),
            ],
            starter_code_language: "javascript".into(),
            starter_code_filename: "crypto_vault.js".into(),
            starter_code: r#"// Basic AES-GCM Encrypted Record Concept using WebCrypto
async function deriveKey(masterPassword, salt) {
  const enc = new TextEncoder();
  const baseKey = await crypto.subtle.importKey(
    'raw', enc.encode(masterPassword), 'PBKDF2', false, ['deriveKey']
  );
  return crypto.subtle.deriveKey(
    { name: 'PBKDF2', salt, iterations: 100000, hash: 'SHA-256' },
    baseKey, { name: 'AES-GCM', length: 256 }, false, ['encrypt', 'decrypt']
  );
}

export async function encryptSecret(secretText, key) {
  const iv = crypto.getRandomValues(new Uint8Array(12));
  const encoded = new TextEncoder().encode(secretText);
  const ciphertext = await crypto.subtle.encrypt({ name: 'AES-GCM', iv }, key, encoded);
  return { iv: Array.from(iv), data: Array.from(new Uint8Array(ciphertext)) };
}
"#.into(),
            documentation: r#"# Encrypted Vault & Password Keeper

Hardware-backed security and zero-knowledge architecture.
"#.into(),
        },

        // 6. Intermediate Mobile 3
        ProjectIdea {
            id: "mob-int-3".into(),
            title: "Split Bill & Expense Group Settler".into(),
            domain: Domain::MobileApp,
            difficulty: Difficulty::Intermediate,
            description: "Expense sharing app that minimizes debt transactions among travel buddies using greedy graph debt simplification algorithms.".into(),
            requirements: vec![
                "Group expense logging with unequal split ratios and custom payers".into(),
                "Debt simplification algorithm (min-flow / greedy cash settlement)".into(),
                "Currency support with conversion multipliers".into(),
                "PDF summary receipt generation for trips".into(),
            ],
            technologies: vec!["React Native / Flutter".into(), "Graph Algorithms".into(), "SQLite / Realm".into()],
            duration: "14 - 20 hours".into(),
            steps: vec![
                "Step 1: Design ledger data model (User, Group, Expense, Share).".into(),
                "Step 2: Calculate net balance for each participant.".into(),
                "Step 3: Implement greedy debt settlement algorithm matching max creditor with max debtor.".into(),
                "Step 4: Build graphical settlement summary card showing 'Who pays Who'.".into(),
            ],
            starter_code_language: "javascript".into(),
            starter_code_filename: "debt_settler.js".into(),
            starter_code: r#"// Greedy Debt Simplification Algorithm
export function simplifyDebts(balances) {
  // balances: { Alice: -20, Bob: 50, Charlie: -30 }
  const creditors = [];
  const debtors = [];
  for (const [person, amount] of Object.entries(balances)) {
    if (amount > 0.01) creditors.push({ person, amount });
    else if (amount < -0.01) debtors.push({ person, amount: -amount });
  }

  const transactions = [];
  let i = 0, j = 0;
  while (i < creditors.length && j < debtors.length) {
    const minAmount = Math.min(creditors[i].amount, debtors[j].amount);
    transactions.push({ from: debtors[j].person, to: creditors[i].person, amount: minAmount.toFixed(2) });
    creditors[i].amount -= minAmount;
    debtors[j].amount -= minAmount;
    if (creditors[i].amount < 0.01) i++;
    if (debtors[j].amount < 0.01) j++;
  }
  return transactions;
}
"#.into(),
            documentation: r#"# Split Bill & Expense Group Settler

Minimizes the number of cash transfers needed to settle all debts in a group.
"#.into(),
        },

        // 7. Advanced Mobile 1
        ProjectIdea {
            id: "mob-adv-1".into(),
            title: "On-Device AR Furniture Room Placer".into(),
            domain: Domain::MobileApp,
            difficulty: Difficulty::Advanced,
            description: "Augmented Reality app (ARKit/ARCore) that detects floor planes, allows users to place realistic 3D furniture models, and tests room layouts with lighting estimation.".into(),
            requirements: vec![
                "Horizontal and vertical surface plane detection with feature point tracking".into(),
                "3D GLTF / USDZ model loading, rotation, and multi-touch pinch scaling".into(),
                "Ambient lighting estimation dynamically updating 3D model shadows".into(),
                "Snapshot capture and room dimension ruler measuring tool".into(),
            ],
            technologies: vec!["ARKit / ARCore".into(), "SceneKit / Filament / Unity".into(), "3D Math & Vectors".into()],
            duration: "35 - 55 hours".into(),
            steps: vec![
                "Step 1: Configure ARSession and request camera tracking permissions.".into(),
                "Step 2: Implement ARPlaneAnchor listener to visually render detected flat surfaces.".into(),
                "Step 3: Raycast touch screen points onto detected AR world planes.".into(),
                "Step 4: Instantiate 3D model node at raycast hit point.".into(),
                "Step 5: Apply directional light estimations from ambient scene luminance.".into(),
            ],
            starter_code_language: "swift".into(),
            starter_code_filename: "ARViewController.swift".into(),
            starter_code: r#"// Swift ARKit Plane Placement snippet
import UIKit
import SceneKit
import ARKit

class ARViewController: UIViewController, ARSCNViewDelegate {
    @IBOutlet var sceneView: ARSCNView!

    override func viewDidLoad() {
        super.viewDidLoad()
        sceneView.delegate = self
        let config = ARWorldTrackingConfiguration()
        config.planeDetection = [.horizontal]
        config.isLightEstimationEnabled = true
        sceneView.session.run(config)
    }

    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        guard let touch = touches.first else { return }
        let location = touch.location(in: sceneView)
        
        let query = sceneView.raycastQuery(from: location, allowing: .estimatedPlane, alignment: .horizontal)
        if let query = query, let result = sceneView.session.raycast(query).first {
            placeModel(at: result.worldTransform)
        }
    }

    func placeModel(at transform: simd_float4x4) {
        let box = SCNBox(width: 0.3, height: 0.3, length: 0.3, chamferRadius: 0.02)
        let node = SCNNode(geometry: box)
        node.simdTransform = transform
        sceneView.scene.rootNode.addChildNode(node)
    }
}
"#.into(),
            documentation: r#"# On-Device AR Furniture Room Placer

Native augmented reality with precise raycasting and lighting estimation.
"#.into(),
        },

        // 8. Advanced Mobile 2
        ProjectIdea {
            id: "mob-adv-2".into(),
            title: "On-Device Real-time Object Detection & OCR Scanner".into(),
            domain: Domain::MobileApp,
            difficulty: Difficulty::Advanced,
            description: "Edge machine learning app using TensorFlow Lite / CoreML to run real-time bounding box object detection and document text scanning at 30+ FPS without internet.".into(),
            requirements: vec![
                "Live camera frame buffer processing pipeline (YUV to RGB conversion)".into(),
                "Quantized YOLO / MobileNet model inference run entirely on device NPU/GPU".into(),
                "Bounding box overlay canvas with confidence threshold filtering".into(),
                "Document edge perspective correction and text OCR extraction".into(),
            ],
            technologies: vec!["TensorFlow Lite / CoreML".into(), "Vision API".into(), "OpenCV".into(), "Kotlin / Swift".into()],
            duration: "30 - 45 hours".into(),
            steps: vec![
                "Step 1: Set up Camera2 / AVFoundation live capture output stream.".into(),
                "Step 2: Quantize pre-trained MobileNet / YOLO model to .tflite / .mlmodel format.".into(),
                "Step 3: Execute model prediction on image tensor buffer.".into(),
                "Step 4: Apply Non-Maximum Suppression (NMS) to eliminate duplicate boxes.".into(),
                "Step 5: Draw detected objects and OCR text overlay directly on camera view.".into(),
            ],
            starter_code_language: "kotlin".into(),
            starter_code_filename: "DetectorAnalyzer.kt".into(),
            starter_code: r#"// Kotlin CameraX ImageAnalysis + TFLite snippet
import androidx.camera.core.ImageAnalysis
import androidx.camera.core.ImageProxy

class DetectorAnalyzer(private val onResult: (List<String>) -> Unit) : ImageAnalysis.Analyzer {
    override fun analyze(imageProxy: ImageProxy) {
        val mediaImage = imageProxy.image
        if (mediaImage != null) {
            // Process buffer with TFLite Interpreter / ML Kit Vision
            // val inputBuffer = convertToByteBuffer(mediaImage)
            // val outputs = interpreter.run(inputBuffer)
            onResult(listOf("Detected: Notebook (94%)", "Detected: Pen (88%)"))
        }
        imageProxy.close()
    }
}
"#.into(),
            documentation: r#"# Real-time Edge Object Detector

Low-latency machine learning inference running natively on mobile hardware accelerators.
"#.into(),
        },

        // 9. Advanced Mobile 3
        ProjectIdea {
            id: "mob-adv-3".into(),
            title: "Mesh Chat: Peer-to-Peer Bluetooth Low Energy Messenger".into(),
            domain: Domain::MobileApp,
            difficulty: Difficulty::Advanced,
            description: "Off-grid messaging app that forms ad-hoc multi-hop mesh networks over Bluetooth Low Energy (BLE) and Wi-Fi Direct when cellular networks fail.".into(),
            requirements: vec![
                "BLE Central and Peripheral roles running concurrently on each device".into(),
                "Multi-hop message forwarding protocol with TTL (Time-To-Live) and flood prevention".into(),
                "End-to-end encrypted packet transmission with Curve25519 key pairs".into(),
                "Automatic peer discovery and topological node routing table".into(),
            ],
            technologies: vec!["Bluetooth Low Energy (BLE)".into(), "Wi-Fi Direct / MultipeerConnectivity".into(), "Cryptography (NaCl)".into(), "Kotlin / Swift".into()],
            duration: "40 - 60 hours".into(),
            steps: vec![
                "Step 1: Define custom BLE Service UUID and Characteristic read/write descriptors.".into(),
                "Step 2: Implement continuous BLE advertising and scanning loop.".into(),
                "Step 3: Build packet framing structure: [Header, SenderID, RecipientID, TTL, Nonce, Ciphertext].".into(),
                "Step 4: Implement gossip / flood routing algorithm with message deduplication hash set.".into(),
                "Step 5: Handle graceful connection drops and background peripheral wakeup.".into(),
            ],
            starter_code_language: "swift".into(),
            starter_code_filename: "BLEMeshManager.swift".into(),
            starter_code: r#"// Swift CoreBluetooth Peripheral & Central initialization
import CoreBluetooth

class BLEMeshManager: NSObject, CBCentralManagerDelegate, CBPeripheralManagerDelegate {
    var centralManager: CBCentralManager!
    var peripheralManager: CBPeripheralManager!
    let SERVICE_UUID = CBUUID(string: "9F1B3E00-7B1A-4C21-A3C9-9A0B8D4E12F0")
    var seenPacketIds = Set<String>()

    func start() {
        centralManager = CBCentralManager(delegate: self, queue: nil)
        peripheralManager = CBPeripheralManager(delegate: self, queue: nil)
    }

    func centralManagerDidUpdateState(_ central: CBCentralManager) {
        if central.state == .poweredOn {
            central.scanForPeripherals(withServices: [SERVICE_UUID], options: nil)
        }
    }

    func peripheralManagerDidUpdateState(_ peripheral: CBPeripheralManager) {
        if peripheral.state == .poweredOn {
            let service = CBMutableService(type: SERVICE_UUID, primary: true)
            peripheral.add(service)
            peripheral.startAdvertising([CBAdvertisementDataServiceUUIDsKey: [SERVICE_UUID]])
        }
    }
}
"#.into(),
            documentation: r#"# Mesh Chat: Peer-to-Peer BLE Messenger

Decentralized disaster-resilient communications network.
"#.into(),
        },

        // 10. Intermediate Mobile 4
        ProjectIdea {
            id: "mob-int-4".into(),
            title: "Micro-Journal with Mood AI Sentiment Analysis".into(),
            domain: Domain::MobileApp,
            difficulty: Difficulty::Intermediate,
            description: "Daily micro-journaling diary that evaluates emotional sentiment from your entries, charting your happiness, gratitude, and stress levels over time.".into(),
            requirements: vec![
                "Clean rich text and photo journal entry composer".into(),
                "AFINN / VADER or on-device sentiment scoring (-5 to +5 score)".into(),
                "Interactive mood timeline charts and weekly emotional summaries".into(),
                "Encrypted local SQLite database with biometric fingerprint protection".into(),
            ],
            technologies: vec!["React Native / Flutter".into(), "Sentiment Analysis".into(), "SVG Charting".into(), "SQLite".into()],
            duration: "15 - 22 hours".into(),
            steps: vec![
                "Step 1: Build journal entry compose screen with mood emoji picker.".into(),
                "Step 2: Implement natural language sentiment scoring on typed text.".into(),
                "Step 3: Save mood scores and journal text to SQLite database.".into(),
                "Step 4: Draw interactive line graphs of emotional variance over 30 days.".into(),
            ],
            starter_code_language: "javascript".into(),
            starter_code_filename: "sentiment.js".into(),
            starter_code: r#"// Basic Lexicon-based Sentiment Scoring
const sentimentLexicon = {
  great: 3, wonderful: 4, happy: 3, excited: 3, productive: 2,
  sad: -3, anxious: -2, stressed: -3, tired: -1, terrible: -4
};

export function scoreJournalEntry(text) {
  const words = text.toLowerCase().match(/\b[a-z]+\b/g) || [];
  let score = 0;
  let matches = 0;

  for (const word of words) {
    if (sentimentLexicon[word]) {
      score += sentimentLexicon[word];
      matches++;
    }
  }

  const normalized = matches > 0 ? (score / matches) : 0;
  return {
    rawScore: score,
    sentiment: normalized > 0.5 ? 'Positive' : normalized < -0.5 ? 'Negative' : 'Neutral'
  };
}
"#.into(),
            documentation: r#"# Micro-Journal with Mood Sentiment Analysis

Track emotional well-being over time with automated text analytics.
"#.into(),
        },
    ]
}
