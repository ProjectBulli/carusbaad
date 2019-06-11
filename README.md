#Car USB Android Auto Daemon

#Setup
 * checkout from ...
 * Install native dependencies:
   * Debian/Ubuntu: `sudo apt-get install libusb-1.0-0-dev`
   
 * build
   * `./build.release.sh`


2019-06-08 18:15:42.775 25141-11287/? E/CAR.GAL.GAL: Received unexpected message of type 36
2019-06-08 18:15:42.776 25141-11286/? W/CAR.GAL.GAL: ReaderThread: crashing with exception
    java.nio.BufferOverflowException
        at java.nio.DirectByteBuffer.put(DirectByteBuffer.java:291)
        at oid.write(:com.google.android.gms@17455039@17.4.55 (100408-248795830):70)
        at oia.run(:com.google.android.gms@17455039@17.4.55 (100408-248795830):5)
2019-06-08 18:15:42.776 25141-25141/? E/CAR.SETUP.SERVICE: PROTOCOL_IO_ERROR: io error
2019-06-08 18:15:42.777 25141-25141/? D/CAR.GAL.SNOOP: Shutdown GAL Snoop
2019-06-08 18:15:42.777 25141-25141/? D/CAR.GAL.SNOOP: Clear the buffer
2019-06-08 18:15:42.777 25141-25141/? D/CAR.SETUP.SERVICE: Closing fd: {ParcelFileDescriptor: java.io.FileDescriptor@7ceac9b}
2019-06-08 18:15:44.826 17000-17074/? I/WorkerManager: dispose()
