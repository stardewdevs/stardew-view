package io.stardew.view;

import android.app.Activity;
import android.os.Bundle;
import android.view.SurfaceView;

public class StardewViewActivity extends Activity {

    private SurfaceView surfaceView;

    static {
        System.loadLibrary("stardew_view");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        surfaceView = new SurfaceView(this);
        setContentView(surfaceView);
    }

    @Override
    protected void onResume() {
        super.onResume();
        final SurfaceView view = surfaceView;
        if (view != null && view.getHolder() != null && view.getHolder().getSurface().isValid()) {
            StardewViewNative.nativeInit(view.getHolder().getSurface(), view.getWidth(), view.getHeight());
        }
    }

    @Override
    protected void onDestroy() {
        super.onDestroy();
        StardewViewNative.nativePresent(0L);
    }

    public static final class StardewViewNative {
        public static native long nativeInit(Object surface, int width, int height);
        public static native void nativeRender(long handle, Object terminalState);
        public static native void nativePresent(long handle);
        public static native void nativeResize(long handle, int width, int height);
    }
}